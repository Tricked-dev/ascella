use crate::prelude::*;

#[derive(Deserialize, Apiv2Schema, Clone)]
pub struct EmbedData {
  pub title: Option<String>,
  #[allow(dead_code)]
  pub link: Option<String>,
  pub url: Option<String>,
  pub description: Option<String>,
  pub color: Option<String>,
}
