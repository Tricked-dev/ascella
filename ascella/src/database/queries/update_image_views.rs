use crate::prelude::*;

pub async fn exec(id: i32) -> Result<()> {
  get_tokio_postgres().await.query_one("UPDATE images SET views = views + 1 WHERE id = $1", &[&id]).await?;
  Ok(())
}
