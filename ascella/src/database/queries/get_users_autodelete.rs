use crate::prelude::*;

pub async fn exec() -> Result<Vec<(i32, i32, String)>> {
  let rows = get_tokio_postgres().await.query("SELECT id,autodelete,name FROM users WHERE autodelete IS NOT null", &[]).await?;

  Ok(rows.iter().map(|row| (row.get("id"), row.get("autodelete"), row.get("name"))).collect())
}
