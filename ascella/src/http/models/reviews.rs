use crate::prelude::*;

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct GetReviewsResponse(pub Vec<Comment>);
apply_responders!(GetReviewsResponse);
