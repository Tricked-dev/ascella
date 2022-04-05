use crate::prelude::*;
#[derive(Deserialize)]
struct ImageData {
  pub image_id: i32,
}

#[api_v2_operation]
#[post("/public")]
pub async fn post(req: HttpRequest, body: web::Bytes) -> Result<HttpResponse, Error> {
  if let Ok(user) = validate_request(&req).await {
    let result = from_str(std::str::from_utf8(&body).unwrap());
    let data: ImageData = match result {
      Ok(v) => v,
      _ => return Err(Error::BadRequest),
    };

    let image = make_public::exec(user.id, data.image_id).await;

    match image {
      Ok(_) => Ok(HttpResponse::Ok().json(&send_message(200, true, "Image is now public"))),
      _ => Err(Error::BadRequest),
    }
  } else {
    Err(Error::NotAuthorized)
  }
}
