use crate::http::models::public::ImageData;
use crate::prelude::*;

/// make a image public
///
/// Make a image public this endpoint is useless atm
#[api_v2_operation(tags(Dashboard), consumes = "application/json", produces = "application/json")]
#[post("/public")]
pub async fn post(body: web::Bytes, user: AccessToken) -> Result<SendMessage, Error> {
  let result = from_str(std::str::from_utf8(&body).unwrap());
  let data: ImageData = match result {
    Ok(v) => v,
    _ => return Err(Error::BadRequest),
  };

  let image = make_public::exec(user.id(), data.image_id).await;

  match image {
    Ok(_) => Ok(SendMessage::new(200, true, "Image is now public")),
    _ => Err(Error::BadRequest),
  }
}
