use crate::{bot::commands::stats::AscellaStats, prelude::*};

#[api_v2_operation(tags(Etc), summary = "get ascella stats", description = "provides some cool stats about ascella to use", produces = "application/json")]
#[post("/stats.json")]
pub async fn get() -> Result<AscellaStats, Error> {
  Ok(AscellaStats::new_with_stats().await)
}

apply_responders!(AscellaStats);
