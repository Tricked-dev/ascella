use crate::prelude::*;

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct Domain {
  pub(crate) apex: bool,
  pub(crate) owner: i32,
  pub(crate) domain: String,
}
