use ascella_util::{
  image_effects::{apply_effects, Attrs},
  validate_request_upload,
};

use crate::routes::prelude::*;

#[post("/upload")]
pub async fn post(req: HttpRequest, mut payload: Multipart) -> Result<HttpResponse, Error> {
  let mut end_effects: Vec<Attrs> = vec![];
  if let Ok((data, effects)) = validate_request_upload(&req).await {
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
          if s == "image/png" {
            // if let Some(effect) = effects.map(|v| v.to_str()) {
            if let Ok(effect) = effects {
              let effects = effect.split(',').collect::<Vec<&str>>();
              for effect in effects {
                let ff = effect.split('=').collect::<Vec<&str>>();
                match ff[0] {
                  "cali" => end_effects.push(Attrs::Cali),
                  "dramatic" => end_effects.push(Attrs::Dramatic),
                  "filter" => end_effects.push(Attrs::Filter(ff[1].to_string())),
                  "firenze" => end_effects.push(Attrs::Firenze),
                  "golden" => end_effects.push(Attrs::Golden),
                  "lix" => end_effects.push(Attrs::Lix),
                  "lofi" => end_effects.push(Attrs::Lofi),
                  "neue" => end_effects.push(Attrs::Neue),
                  "obsidian" => end_effects.push(Attrs::Obsidian),
                  "pastel_pink" => end_effects.push(Attrs::PastelPink),
                  "ryo" => end_effects.push(Attrs::Ryo),
                  _ => {}
                };
              }
            }

            // };
          }
        }
        _ => return Err(Error::FileTypeNotAllowed),
      };

      let img = create_image::exec(data.id, content_type, ran_str()).await.unwrap();

      create_dir_all(format!("images/{}", data.id)).await.unwrap();
      let dest = format!("images/{}/{}", data.id, img.id,);
      if !end_effects.is_empty() && end_effects.len() < 10 {
        apply_effects(buf, end_effects, dest)?;
      } else {
        let mut f = web::block(|| std::fs::File::create(dest))
          .await
          .map_err(|_| Error::BadRequest)?
          .map_err(|_| Error::BadRequest)?;

        web::block(move || f.write_all(&buf))
          .await
          .map_err(|_| Error::BadRequest)?
          .map_err(|_| Error::BadRequest)?;
      }

      actix_web::rt::spawn(
        //Used to prevent abuse - sorry its needed
        send_text_webhook(format!(
          "**[IMAGE]** [image](<https://ascella.wtf/v2/ascella/view/{image}>) **[OWNER]** {name} ({id})",
          image = &img.vanity,
          name = &data.name,
          id = &data.id
        )),
      );
      Ok(HttpResponse::Ok().json(&upload_success(&img.vanity, &data.domain)))
    } else {
      Err(Error::BadRequest)
    }
  } else {
    Err(Error::NotAuthorized)
  }
}
