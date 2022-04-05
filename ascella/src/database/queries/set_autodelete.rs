use crate::database::queries::prelude::*;

pub async fn exec(id: i32, time: i32) -> Result<()> {
  get_tokio_postgres()
    .await
    .query("UPDATE users SET autodelete = $1 WHERE id = $2", &[&time, &id])
    .await?;
  Ok(())
}
