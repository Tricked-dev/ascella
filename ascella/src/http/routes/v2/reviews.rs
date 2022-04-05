use crate::bot::bot::REVIEWS;
use crate::prelude::*;

#[api_v2_operation]
#[get("/reviews")]
pub async fn get() -> Result<HttpResponse, Error> {
  Ok(HttpResponse::Ok().json(REVIEWS.get().unwrap()))
}
