use crate::routes::prelude::*;
use cached::proc_macro::cached;
use std::io;

#[cached(size = 100, time = 512, result = true)]
fn get_image(path: String) -> io::Result<Vec<u8>> {
  read(path)
}

#[get("/view/{image}")]
pub async fn get(image: web::Path<String>) -> Result<HttpResponse, Error> {
  let data = get_image_vanity_only::exec(image.to_string()).await;
  if let Ok(image) = data {
    let data = get_image(format!("./images/{}/{}", image.owner, image.id));
    match data {
      Ok(data) => Ok(
        HttpResponse::Ok()
          .append_header(("content-type", image.content_type))
          .append_header(("cache-control", "public, max-age=604800, immutable"))
          .body(data),
      ),
      _ => Err(Error::NotFound),
    }
  } else {
    Err(Error::NotFound)
  }
}
