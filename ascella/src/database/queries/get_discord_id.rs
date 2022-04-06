use crate::prelude::*;

#[cached(size = 100, time = 120, result = true)]
pub async fn exec(id: i32) -> Result<(String, String)> {
  let row = get_tokio_postgres().await.query_one("SELECT discord_id, name FROM users WHERE id = $1 LIMIT 1", &[&id]).await?;

  Ok((row.get("discord_id"), row.get("name")))
}
