use crate::prelude::*;

#[derive(Deserialize, Apiv2Schema, Clone)]
pub struct QueryData {
  pub skip: i32,
}
