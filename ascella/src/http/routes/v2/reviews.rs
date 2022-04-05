use super::super::prelude::*;

use crate::bot::bot::REVIEWS;

#[api_v2_operation]
#[get("/reviews")]
pub async fn get() -> Result<HttpResponse, Error> {
  Ok(HttpResponse::Ok().json(REVIEWS.get().unwrap()))
}
