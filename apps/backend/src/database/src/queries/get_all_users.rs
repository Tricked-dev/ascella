use crate::queries::prelude::*;

pub async fn exec() -> Result<Vec<Users>> {
  let row = get_tokio_postgres().await.query("SELECT * FROM users", &[]).await?;

  Ok(row.iter().map(|row| Users::from_row_ref(row).unwrap()).collect::<Vec<_>>())
}
