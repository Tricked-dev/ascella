use crate::queries::prelude::*;
pub async fn exec(id: i32, val: bool) -> Result<()> {
  get_tokio_postgres()
    .await
    .query("UPDATE users SET emojis = $1 WHERE id = $2", &[&val, &id])
    .await?;
  Ok(())
}
