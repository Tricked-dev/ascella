use crate::prelude::*;
/// ReportData
///
/// Data needed for reporting
#[derive(Deserialize, Apiv2Schema, Clone, TS)]
#[ts(export)]
pub struct ReportData {
  pub(crate) id: i32,
}
