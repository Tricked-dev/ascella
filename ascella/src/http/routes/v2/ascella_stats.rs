use crate::bot::commands::stats::AscellaStats;
use crate::prelude::*;

/// get ascella stats
///
/// provides some cool stats about ascella to use somewhere
#[api_v2_operation(tags(Etc), produces = "application/json")]
#[post("/stats.json")]
pub async fn get() -> Result<OkResponse<AscellaStats>, Error> {
  Ok(OkResponse(AscellaStats::new_with_stats().await))
}
