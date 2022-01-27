use async_trait::async_trait;
use bytes::{Buf, Bytes};
use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;
use sha2::Digest;
use std::io::Read;
use thiserror::Error;

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

use lazy_static::lazy_static;

lazy_static! {
  pub static ref S3: S3Host = {
    S3Host::new(
      "ascella",
      "EU1",
      "https://gateway.eu1.storjshare.io",
      &dotenv::var("S3_ID").unwrap(),
      &dotenv::var("S3_SECRET").unwrap(),
    )
    .unwrap()
  };
}

impl S3Host {
  pub fn new(bucket_name: &str, bucket_region: &str, url: &str, access_token: &str, secret: &str) -> Result<S3Host, FileHostingError> {
    let mut bucket = Bucket::new(
      bucket_name,
      Region::Custom {
        region: bucket_region.to_string(),
        endpoint: url.to_string(),
      },
      Credentials::new(Some(access_token), Some(secret), None, None, None).map_err(|_| FileHostingError::AnError)?,
    )
    .map_err(|_| FileHostingError::AnError)?;

    bucket.add_header("x-amz-acl", "public-read");

    Ok(S3Host { bucket })
  }
}

impl S3Host {
  pub async fn upload_file(&self, content_type: &str, file_name: &str, file_bytes: Bytes) -> Result<UploadFileData, FileHostingError> {
    let content_sha1 = sha1::Sha1::from(&file_bytes).hexdigest();
    let content_sha512 = format!("{:x}", sha2::Sha512::digest(file_bytes.bytes()));

    self
      .bucket
      .put_object_with_content_type(format!("/{}", file_name), &file_bytes.bytes(), content_type)
      .await
      .map_err(|_| FileHostingError::AnError)?;

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

  pub async fn get_file(&self, s: String) -> Result<Vec<u8>, FileHostingError> {
    Ok(self.bucket.get_object(s).await.map_err(|_| FileHostingError::AnError)?.0)
  }

  pub async fn delete_file_version(&self, file_id: &str, file_name: &str) -> Result<DeleteFileData, FileHostingError> {
    self
      .bucket
      .delete_object(format!("/{}", file_name))
      .await
      .map_err(|_| FileHostingError::AnError)?;

    Ok(DeleteFileData {
      file_id: file_id.to_string(),
      file_name: file_name.to_string(),
    })
  }
}
