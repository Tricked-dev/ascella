use ascella_database::s3::S3;
use ascella_util::validate_request_upload;

use crate::routes::prelude::*;

#[post("/upload")]
pub async fn post(req: HttpRequest, mut payload: Multipart) -> Result<HttpResponse, Error> {
  if let Ok((data, _)) = validate_request_upload(&req).await {
    if let Ok(Some(mut field)) = payload.try_next().await {
      let mut file_size: usize = 0;
      let mut buf: Vec<u8> = Vec::new();
      while let Some(chunk) = field.next().await {
        let data = chunk.map_err(|_| Error::BadRequest)?;
        file_size += data.len();

        if file_size > 1000000 {
          return Err(Error::BadRequest);
        }

        buf.append(&mut data.to_vec());
      }

      let content_type = tree_magic::from_u8(&buf);
      let s = &content_type[..];

      match s {
        "image/jpeg" | "image/png" | "image/gif" | "image/webp" => {
          if s == "image/jpeg" {
            let mut bytes: Vec<u8> = Vec::new();

            // Re-encode JPEGs to remove EXIF data.
            ImageReader::new(Cursor::new(buf))
              .with_guessed_format()
              .map_err(|_| Error::IOError)?
              .decode()
              .map_err(|_| Error::IOError)?
              .write_to(&mut bytes, image::ImageOutputFormat::Jpeg(80))
              .map_err(|_| Error::IOError)?;

            buf = bytes;
          }
        }
        _ => return Err(Error::FileTypeNotAllowed),
      };

      let img = create_image::exec(data.id, content_type.clone(), ran_str()).await.unwrap();

      // create_dir_all(format!("images/{}", data.id)).await.unwrap();
      let dest = format!("{}/{}", data.id, img.id,);

      S3.upload_file(&content_type, dest.as_str(), buf.into()).await.map_err(|_| Error::BadRequest)?;

      actix_web::rt::spawn(send_text_webhook(format!(
        "**[IMAGE]** [image](<https://ascella.wtf/v2/ascella/view/{image}>) **[OWNER]** {name} ({id})",
        image = &img.vanity,
        name = &data.name,
        id = &data.id
      )));
      Ok(HttpResponse::Ok().json(&upload_success(&img.vanity, &data.domain)))
    } else {
      Err(Error::BadRequest)
    }
  } else {
    Err(Error::NotAuthorized)
  }
}
