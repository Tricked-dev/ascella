use crate::http::models::reviews::GetReviewsResponse;
use crate::prelude::*;

/// get reviews
///
/// Get ascella reviews!
#[api_v2_operation(tags(Etc), summary = "get reviews", description = "Get ascella reviews!", consumes = "application/json", produces = "application/json")]
#[get("/reviews")]
pub async fn get() -> Result<GetReviewsResponse, Error> {
  Ok(GetReviewsResponse(REVIEWS.get().unwrap_or(&vec![]).to_vec()))
}
