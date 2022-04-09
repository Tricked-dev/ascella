use crate::prelude::*;
/// DomainData
///
/// Set your new domain here!
#[derive(Deserialize, Apiv2Schema, Clone, TS)]
#[ts(export)]
pub struct DomainData {
  pub(crate) domain: String,
}
