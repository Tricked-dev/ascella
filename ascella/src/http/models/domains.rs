use crate::prelude::*;
/// Domain
///
/// A single domain in a domain vector
#[derive(Serialize, Deserialize, Apiv2Schema, TS)]
#[ts(export)]
pub struct Domain {
  /// Weather or not this is a apex domain
  pub(crate) apex: bool,
  /// Id of the user who owns this domain
  pub(crate) owner: i32,
  #[openapi(example = "Ascella.host")]
  pub(crate) domain: String,
}
