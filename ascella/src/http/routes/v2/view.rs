use tokio::sync::Mutex;

use crate::database::s3::get_file;
use crate::http::ImageCache;
use crate::prelude::*;

/// get image
///
/// View a image uploaded to ascella
#[api_v2_operation(tags(Images), consumes = "application/json")]
#[get("/view/{image}.{ext:(gif|webp|jpg|jpeg|png)$}")]
pub async fn get(params: web::Path<(String, String)>, cache: web::Data<Mutex<ImageCache>>) -> Result<HttpResponse, Error> {
  let cache = cache.lock().await.clone();
  let (image, _ext) = params.into_inner();
  if cache.get_key() == image {
    return Ok(
      HttpResponse::Ok()
        .append_header(("content-type", cache.get_content_type()))
        .append_header(("cache-control", "public, max-age=604800, immutable"))
        .body(cache.inner),
    );
  }
  let data = get_image_vanity_only::exec(image).await;
  if let Ok(image) = data {
    let data = get_file(format!("{}/{}", image.owner, image.id)).await;
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
