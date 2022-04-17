use crate::prelude::*;

pub async fn exec(id: i32) -> Result<Images> {
  let row = get_tokio_postgres().await.query_one("SELECT * FROM images WHERE owner = $1 ORDER BY id DESC LIMIT 1", &[&id]).await?;

  Ok(Images::from_row(row).unwrap())
}
