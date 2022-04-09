use crate::prelude::*;
/// Domain
///
/// A single domain in a domain vector
#[derive(Serialize, Deserialize, Apiv2Schema, TS)]
#[ts(export)]
pub struct Domain {
  pub(crate) apex: bool,
  pub(crate) owner: i32,
  pub(crate) domain: String,
}
