// TODO: type this result

use crate::prelude::*;

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct GetReviewsResponse(Vec<Comment>);

#[api_v2_operation(tags(Etc), summary = "get reviews", description = "Get ascella reviews!", consumes = "application/json", produces = "application/json")]
#[get("/reviews")]
pub async fn get() -> Result<GetReviewsResponse, Error> {
  Ok(GetReviewsResponse(REVIEWS.get().unwrap().to_vec()))
}

apply_responders!(GetReviewsResponse);
