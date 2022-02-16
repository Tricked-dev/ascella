use crate::queries::prelude::*;
pub async fn exec(domain: &'static str, discord_id: String, key: String, name: String) -> Result<Users> {
  let row = get_tokio_postgres()
    .await
    .query_one(
      "INSERT INTO users(domain, discord_id, key,name) VALUES($1,$2,$3,$4) RETURNING *",
      &[&domain, &discord_id, &key, &name],
    )
    .await?;

  Ok(Users::from_row(row).unwrap())
}
