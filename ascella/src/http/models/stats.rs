use crate::prelude::*;

#[derive(Deserialize, Apiv2Schema, Clone, Serialize)]
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
apply_responders!(StatsResponse);
