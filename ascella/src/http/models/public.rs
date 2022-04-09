use crate::prelude::*;

/// ImageData
///
/// stuff
#[derive(Deserialize, TS, Serialize)]
#[ts(export)]
pub struct ImageData {
  pub image_id: i32,
}
