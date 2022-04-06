use crate::prelude::*;
pub async fn exec(user: i32, code: String) -> Result<()> {
  get_tokio_postgres().await.query("INSERT INTO codes(owner,key) VALUES($1,$2)", &[&user, &code]).await?;
  Ok(())
}
