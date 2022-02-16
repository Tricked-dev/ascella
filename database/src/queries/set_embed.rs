use crate::queries::prelude::*;
pub async fn exec(id: i32, description: Option<String>, title: Option<String>, url: Option<String>, color: Option<String>) -> Result<()> {
  let pg = get_tokio_postgres().await;
  pg.query("DELETE FROM embeds WHERE owner = $1", &[&id]).await?;
  pg.query(
    "INSERT INTO embeds(color,description,owner,title,url) VALUES($1,$2,$3,$4,$5)",
    &[&color, &description, &id, &title, &url],
  )
  .await?;
  Ok(())
}
