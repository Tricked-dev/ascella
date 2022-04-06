use crate::prelude::*;

pub async fn exec(discord: String, domain: String) -> Result<()> {
  get_tokio_postgres().await.query("UPDATE users SET domain = $1 WHERE discord_id = $2", &[&domain, &discord]).await?;
  Ok(())
}
