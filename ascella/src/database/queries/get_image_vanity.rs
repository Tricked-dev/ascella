use crate::database::queries::prelude::*;

#[cached(size = 100, time = 120, result = true)]
pub async fn exec(vanity: String, owner: i32) -> Result<Images> {
  let row = get_tokio_postgres()
    .await
    .query_one("SELECT * FROM images WHERE owner = $1 AND vanity = $2 LIMIT 1", &[&owner, &vanity])
    .await?;

  Ok(Images::from_row(row).unwrap())
}
