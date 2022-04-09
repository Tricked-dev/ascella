use crate::prelude::*;
//// Stats
///
/// View stats of a image
#[derive(Deserialize, Apiv2Schema, Clone, Serialize, TS)]
#[ts(export)]
pub struct StatsResponse {
  pub(crate) user_name: String,
  pub(crate) user_id: i32,
  pub(crate) id: i32,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) redirect: Option<String>,
  pub(crate) content_type: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) image_size: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) embed: Option<Embeds>,
}
