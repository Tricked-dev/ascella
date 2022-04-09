use crate::prelude::*;

/// QueryData
///
/// The data required to view stuff
#[derive(Deserialize, Apiv2Schema, Clone, TS)]
pub struct QueryData {
  pub skip: i32,
}
