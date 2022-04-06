use crate::prelude::*;
#[derive(Deserialize)]
struct ImageData {
  pub image_id: i32,
}

#[api_v2_operation(
  tags(Dashboard),
  summary = "make image public",
  description = "Make a image public this endpoint is useless atm",
  consumes = "application/json, text/plain",
  produces = "application/json"
)]
#[post("/public")]
pub async fn post(_req: HttpRequest, body: web::Bytes, user: AccessToken) -> Result<SendMessage, Error> {
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
