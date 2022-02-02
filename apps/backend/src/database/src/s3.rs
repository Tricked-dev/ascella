use anyhow::Result;
use async_trait::async_trait;
use bytes::{Buf, Bytes};
use cached::{proc_macro::cached, Cached, CachedAsync};
use parking_lot::Mutex;
use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;
use s3::serde_types::HeadObjectResult;
use sha2::Digest;
use std::io::Read;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::RwLock;
use std::time::Duration;
use thiserror::Error;
use ttl_cache::TtlCache;

#[derive(Error, Debug)]
pub enum FileHostingError {
    #[error("Error while accessing the data from backblaze")]
    FileSystemError(#[from] std::io::Error),
    #[error("Invalid Filename")]
    InvalidFilename,
    #[error("An error occured")]
    AnError,
}

#[derive(Debug, Clone)]
pub struct UploadFileData {
    pub file_id: String,
    pub file_name: String,
    pub content_length: u32,
    pub content_sha512: String,
    pub content_sha1: String,
    pub content_md5: Option<String>,
    pub content_type: String,
    pub upload_timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct DeleteFileData {
    pub file_id: String,
    pub file_name: String,
}
pub struct S3Host {
    bucket: Bucket,
}
//head_object
use lazy_static::lazy_static;

lazy_static! {
    pub static ref S3: Arc<S3Host> = {
        Arc::new(
            S3Host::new(
                &dotenv::var("S3_BUCKET").unwrap_or("ascella".to_owned()),
                "EU1",
                "https://gateway.eu1.storjshare.io",
                &dotenv::var("S3_ID").unwrap(),
                &dotenv::var("S3_SECRET").unwrap(),
            )
            .unwrap(),
        )
    };
    pub static ref CACHE: Arc<Mutex<TtlCache<String, Vec<u8>>>> =
        Arc::new(Mutex::new(TtlCache::new(20)));
}

impl S3Host {
    pub fn new(
        bucket_name: &str,
        bucket_region: &str,
        url: &str,
        access_token: &str,
        secret: &str,
    ) -> Result<S3Host, FileHostingError> {
        let mut bucket = Bucket::new(
            bucket_name,
            Region::Custom {
                region: bucket_region.to_string(),
                endpoint: url.to_string(),
            },
            Credentials::new(Some(access_token), Some(secret), None, None, None)
                .map_err(|_| FileHostingError::AnError)?,
        )
        .map_err(|_| FileHostingError::AnError)?;

        bucket.add_header("x-amz-acl", "public-read");

        Ok(S3Host { bucket })
    }
}

#[cached(size = 100, time = 120, result = true)]
pub async fn get_file(s: String) -> Result<Vec<u8>, FileHostingError> {
    let data = S3
        .bucket
        .get_object(s.clone())
        .await
        .map_err(|_| FileHostingError::AnError)?
        .0;
    Ok(data)
}

impl S3Host {
    pub async fn upload_file(
        &self,
        content_type: &str,
        file_name: &str,
        file_bytes: Bytes,
    ) -> Result<UploadFileData, FileHostingError> {
        let content_sha1 = sha1::Sha1::from(&file_bytes).hexdigest();
        let content_sha512 = format!("{:x}", sha2::Sha512::digest(file_bytes.bytes()));

        let file_name_clone = file_name.clone().to_owned();
        let content_type_clone = content_type.clone().to_owned();
        let file_bytes_clone = file_bytes.clone().to_owned();
        tokio::spawn(async move {
            S3.bucket
                .put_object_with_content_type(
                    format!("/{}", &file_name_clone),
                    &file_bytes_clone.bytes(),
                    &content_type_clone,
                )
                .await
                .map_err(|_| FileHostingError::AnError)
                .unwrap();
        });

        Ok(UploadFileData {
            file_id: file_name.to_string(),
            file_name: file_name.to_string(),
            content_length: file_bytes.len() as u32,
            content_sha512,
            content_sha1,
            content_md5: None,
            content_type: content_type.to_string(),
            upload_timestamp: chrono::Utc::now().timestamp_millis() as u64,
        })
    }

    pub async fn delete_file_version(
        &self,
        file_id: &str,
        file_name: &str,
    ) -> Result<DeleteFileData, FileHostingError> {
        self.bucket
            .delete_object(format!("/{}", file_name))
            .await
            .map_err(|_| FileHostingError::AnError)?;

        Ok(DeleteFileData {
            file_id: file_id.to_string(),
            file_name: file_name.to_string(),
        })
    }
    pub async fn metadata(&self, file_name: String) -> Result<HeadObjectResult> {
        let r = self.bucket.head_object(file_name).await?.0;
        Ok(r)
    }
}
