use crate::prelude::*;

/// LangQUery
///
/// Query data for a languages
#[derive(Deserialize, Apiv2Schema, Clone, TS)]
#[ts(export)]
pub struct LangQuery {
  #[openapi(example = "nl")]
  pub lang: Option<String>,
}
