use crate::prelude::*;

pub async fn exec(key: String, user: &i32) -> Result<()> {
  get_tokio_postgres()
    .await
    .execute("UPDATE codes SET claimed_by = $1 WHERE key = $2", &[&user, &key])
    .await?;
  Ok(())
}
