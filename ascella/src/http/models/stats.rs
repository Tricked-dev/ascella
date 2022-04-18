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
  pub(crate) views: i32,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) redirect: Option<String>,
  pub(crate) content_type: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) image_size: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) embed: Option<DisplayEmbed>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Apiv2Schema, TS)]
#[ts(export)]
pub struct DisplayEmbed {
  pub color: Option<String>,
  pub description: Option<String>,
  pub title: Option<String>,
  pub url: Option<String>,
}
