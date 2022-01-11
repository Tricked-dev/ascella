use crate::queries::prelude::*;
pub async fn exec(id: i32, to: String, vanity: String) -> Result<()> {
  let pg = get_tokio_postgres().await;
  let exists = pg.execute("SELECT * FROM images WHERE vanity = $1", &[&vanity]).await?;
  if exists == 1 {
    println!("aa {:#?}", exists);
    return Err(anyhow!(""));
  }
  pg.execute(
    "INSERT INTO images(redirect,owner,vanity,content_type) VALUES($1,$2,$3,'redirect')",
    &[&to, &id, &vanity],
  )
  .await?;

  Ok(())
}
