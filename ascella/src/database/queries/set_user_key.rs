use crate::prelude::*;

pub async fn exec(id: i32, val: String) -> Result<()> {
  get_tokio_postgres().await.query("UPDATE users SET key = $1 WHERE id = $2", &[&val, &id]).await?;
  Ok(())
}
