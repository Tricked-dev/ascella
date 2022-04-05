use crate::prelude::*;

#[cached(size = 100, time = 120, result = true)]
pub async fn exec(code: String) -> Result<Codes> {
  let row = get_tokio_postgres()
    .await
    .query_one("SELECT * FROM codes WHERE key = $1 AND claimed_by IS NULL LIMIT 1", &[&code])
    .await?;

  Ok(Codes::from_row(row).unwrap())
}
