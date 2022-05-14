use crate::prelude::*;
use validator::{Validate, ValidationError};

/// NodifyUser
///
/// Data needed to update the user!
#[derive(Deserialize, Apiv2Schema, Clone, TS, Validate)]
#[ts(export)]
pub struct ModifyUser {
  #[validate(length(min = 3, max = 20))]
  pub name: Option<String>,
  #[validate(length(min = 5, max = 20))]
  pub key: Option<String>,
  #[validate(length(min = 3, max = 100))]
  pub domain: Option<String>,
  #[validate(range(min = 1, max = 265))]
  pub autodelete: Option<i32>,
  #[validate(range(min = 1, max = 265))]
  pub deleteall: Option<i32>,
  #[validate(length(min = 5, max = 20))]
  pub upload_key: Option<String>,
  #[validate(length(min = 2, max = 2))]
  pub lang: Option<String>,
}
