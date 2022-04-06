use crate::prelude::*;

#[cached(size = 100, time = 120, result = true)]
pub async fn exec(id: String) -> Result<String> {
  let row = get_tokio_postgres().await.query_one("SELECT content FROM pastes WHERE id = $1 LIMIT 1", &[&id]).await?;
  Ok(row.get("content"))
}
