use crate::routes::prelude::*;
use ascella_database::s3::get_file;

use paperclip::actix::{
  api_v2_operation,
  web::{self},
};

#[api_v2_operation]
#[get("/view/{image}.{ext:(gif|webp|jpg|jpeg|png)$}")]
pub async fn get(params: web::Path<(String, String)>) -> Result<HttpResponse, Error> {
  let (image, _ext) = params.into_inner();
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
