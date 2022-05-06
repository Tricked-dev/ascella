use crate::prelude::*;

/// LangQUery
///
/// Query data for a languages
#[derive(Deserialize, Apiv2Schema, Clone, TS)]
#[ts(export)]
pub struct LangQuery {
  pub lang: Option<String>,
}
