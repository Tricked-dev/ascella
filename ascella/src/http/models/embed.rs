use crate::prelude::*;

/// EmbedData
///
/// Data for the embed
#[derive(Deserialize, Apiv2Schema, Clone, TS)]
#[ts(export)]
pub struct EmbedData {
  pub title: Option<String>,
  #[allow(dead_code)]
  pub link: Option<String>,
  pub url: Option<String>,
  pub description: Option<String>,
  pub color: Option<String>,
  pub author: Option<String>,
}
