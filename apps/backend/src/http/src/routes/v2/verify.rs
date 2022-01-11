use crate::routes::prelude::*;

#[post("/verify")]
pub async fn post(req: HttpRequest) -> Result<HttpResponse, Error> {
  if let Ok(data) = validate_request(&req).await {
    Ok(HttpResponse::Ok().json(&data))
  } else {
    Err(Error::NotAuthorized)
  }
}
