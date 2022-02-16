use crate::queries::prelude::*;

#[cached(size = 100, time = 120, result = true)]
pub async fn exec(id: i32) -> Result<i64> {
  let row = get_tokio_postgres()
    .await
    .query_one("SELECT count(*) FROM images WHERE owner = $1", &[&id])
    .await?;
  Ok(row.get("count"))
}
