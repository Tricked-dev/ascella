use crate::prelude::*;

/// RedirectData
///
/// Set the redirect
#[derive(Deserialize, Apiv2Schema, Clone, TS)]
#[ts(export)]
pub struct RedirectData {
  #[openapi(example = "google")]
  pub vanity: String,
  #[openapi(example = "https://google.com")]
  pub to: String,
}
