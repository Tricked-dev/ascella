use crate::database::queries::prelude::*;

pub async fn exec(id: i32, val: i32) -> Result<()> {
  get_tokio_postgres()
    .await
    .query("UPDATE users SET url_style = $1 WHERE id = $2", &[&val, &id])
    .await?;
  Ok(())
}
