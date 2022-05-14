use crate::prelude::*;
/// UploadStyleData
///
/// Set your new domain here!
#[derive(Deserialize, Apiv2Schema, Clone, TS)]
#[ts(export)]
pub struct UploadStyleData {
  pub(crate) style: i32,
}
