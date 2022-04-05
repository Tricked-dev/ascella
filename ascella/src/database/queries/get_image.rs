use crate::database::queries::prelude::*;

#[cached(size = 100, time = 120, result = true)]
pub async fn exec(id: String) -> Result<Images> {
  let row = get_tokio_postgres()
    .await
    .query_one("SELECT * FROM images WHERE id = $1 LIMIT 1", &[&id])
    .await?;

  Ok(Images::from_row(row).unwrap())
}
