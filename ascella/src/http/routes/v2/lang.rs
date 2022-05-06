use crate::http::models::lang::LangQuery;
use crate::prelude::*;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "../locales"]
struct Languages;

#[derive(Apiv2Schema, Serialize, Clone, Deserialize, Default)]
pub struct LanguagesResponse {
  data: Value,
}

/// get a language
///
/// Get a language file of ascella
#[api_v2_operation(tags(Etc), produces = "application/json")]
#[get("/language.json")]
pub async fn get(data: web::Query<LangQuery>) -> Result<OkResponse<LanguagesResponse>, Error> {
  if let Some(lang) = &data.lang {
    if let Some(file) = Languages::get(lang) {
      return Ok(OkResponse(LanguagesResponse {
        data: serde_json::from_str(std::str::from_utf8(file.data.as_ref()).unwrap()).unwrap(),
      }));
    }
  }
  Ok(OkResponse(LanguagesResponse {
    data: serde_json::from_str(&serde_json::to_string(&Languages::iter().map(|x| x.to_string()).collect::<Vec<_>>()).unwrap()).unwrap(),
  }))
}
