use crate::prelude::*;

#[derive(Deserialize, Apiv2Schema)]
pub struct Data {
  pub(crate) auth: String,
}
