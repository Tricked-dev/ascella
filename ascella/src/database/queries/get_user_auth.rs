use crate::database::queries::prelude::*;

#[cached(size = 100, time = 120, result = true)]
pub async fn exec(auth: String) -> Result<Users> {
  let row = get_tokio_postgres()
    .await
    .query_one("SELECT * FROM users WHERE upload_key = $1 LIMIT 1", &[&auth])
    .await?;

  Ok(Users::from_row(row).unwrap())
}
