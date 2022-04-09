use crate::prelude::*;
/// Data
///
/// Query string used to download your config
#[derive(Deserialize, Apiv2Schema, TS)]
#[ts(export)]
pub struct Data {
  pub(crate) auth: String,
}
