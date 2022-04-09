use crate::prelude::*;

/// RedirectData
///
/// Set the redirect
#[derive(Deserialize, Apiv2Schema, Clone, TS)]
#[ts(export)]
pub struct RedirectData {
  pub vanity: String,
  pub to: String,
}
