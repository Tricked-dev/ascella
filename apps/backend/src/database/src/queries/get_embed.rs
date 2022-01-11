use crate::queries::prelude::*;

#[cached(size = 100, time = 120, result = true)]
pub async fn exec(id: i32) -> Result<Embeds> {
  let row = get_tokio_postgres()
    .await
    .query_one("SELECT * FROM embeds WHERE owner = $1 LIMIT 1", &[&id])
    .await?;

  Ok(Embeds::from_row(row).unwrap())
}
