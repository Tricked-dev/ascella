use crate::prelude::*;

#[cached(size = 100, time = 120, result = true)]
pub async fn exec(id: i32) -> Result<Users> {
  let row = get_tokio_postgres()
    .await
    .query_one("SELECT * FROM users WHERE id = $1 LIMIT 1", &[&id])
    .await?;

  Ok(Users::from_row(row).unwrap())
}
