use crate::prelude::*;

pub async fn exec(id: i32, description: Option<String>, title: Option<String>, url: Option<String>, color: Option<String>, author: Option<String>) -> Result<()> {
  let pg = get_tokio_postgres().await;
  pg.query("DELETE FROM embeds WHERE owner = $1", &[&id]).await?;
  pg.query(
    "INSERT INTO embeds(color,description,owner,title,url,author) VALUES($1,$2,$3,$4,$5,$6)",
    &[&color, &description, &id, &title, &url, &author],
  )
  .await?;
  Ok(())
}
