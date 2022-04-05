use crate::database::queries::prelude::*;

pub async fn exec(domain: &'static str, discord_id: String, key: String, name: String, upload: String) -> Result<Users> {
  let row = get_tokio_postgres()
    .await
    .query_one(
      "INSERT INTO users(domain, discord_id, key,name,upload_key) VALUES($1,$2,$3,$4,$5) RETURNING *",
      &[&domain, &discord_id, &key, &name, &upload],
    )
    .await?;

  Ok(Users::from_row(row).unwrap())
}
