use crate::prelude::*;

#[derive(Deserialize, Apiv2Schema, Clone)]
pub struct RedirectData {
  pub vanity: String,
  pub to: String,
}
