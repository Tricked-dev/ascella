use crate::prelude::*;

#[derive(Deserialize, Apiv2Schema, Clone)]
pub struct QueryData {
  pub skip: i32,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct GetImagesResponse(pub Vec<SimpleImages>);
apply_responders!(GetImagesResponse);
