use crate::prelude::*;

#[derive(Deserialize, Apiv2Schema, Clone)]
pub struct DomainData {
  pub(crate) domain: String,
}
