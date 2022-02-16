use crate::queries::prelude::*;
pub async fn exec(id: i32, content_type: String, vanity: String) -> Result<Images> {
  let pg = get_tokio_postgres().await;
  let exists = pg.execute("SELECT * FROM images WHERE vanity = $1", &[&vanity]).await?;
  if exists == 1 {
    println!("aa {:#?}", exists);
    return Err(anyhow!(""));
  }
  let row = pg
    .query_one(
      "INSERT INTO images(owner,vanity,content_type) VALUES($1,$2,$3) RETURNING *",
      &[&id, &vanity, &content_type],
    )
    .await?;

  Ok(Images::from_row(row).unwrap())
}
