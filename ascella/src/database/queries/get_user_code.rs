use crate::prelude::*;

#[cached(size = 100, time = 120, result = true)]
pub async fn exec(invite_code: String) -> Result<Users> {
  let row = get_tokio_postgres().await.query_one("SELECT * FROM users WHERE invite_code = $1 LIMIT 1", &[&invite_code]).await?;

  Ok(Users::from_row(row).unwrap())
}
