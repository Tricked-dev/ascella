use crate::prelude::*;

pub async fn exec() -> Result<i64> {
  let row = get_tokio_postgres().await.query_one("SELECT SUM(views) FROM images;", &[]).await?;
  Ok(row.get("sum"))
}
