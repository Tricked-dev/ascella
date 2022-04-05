// TODO: type this result

use crate::bot::bot::REVIEWS;
use crate::prelude::*;

#[api_v2_operation(description = "Get ascella reviews!", consumes = "application/json, text/plain", produces = "application/json")]
#[get("/reviews")]
pub async fn get() -> Result<HttpResponse, Error> {
  Ok(HttpResponse::Ok().json(REVIEWS.get().unwrap()))
}
