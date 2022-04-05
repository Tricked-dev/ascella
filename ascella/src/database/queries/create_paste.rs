use crate::prelude::*;

pub async fn exec(id: String, content: String) -> Result<Pastes> {
  let row = get_tokio_postgres()
    .await
    .query_one("INSERT INTO pastes(content,id) VALUES($1,$2) RETURNING *", &[&content, &id])
    .await?;
  Ok(Pastes::from_row(row).unwrap())
}
