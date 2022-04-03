use crate::routes::prelude::*;
use ascella_bot::bot::REVIEWS;

#[get("/reviews")]
pub async fn get() -> Result<HttpResponse, Error> {
  Ok(HttpResponse::Ok().json(REVIEWS.get().unwrap()))
}
