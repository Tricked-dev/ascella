use crate::prelude::*;

pub async fn exec(domain: &'static str, discord_id: String, key: String, name: String, upload: String, invite_code: String, invited_by: i32) -> Result<Users> {
  let row = get_tokio_postgres()
    .await
    .query_one(
      "INSERT INTO users(domain, discord_id, key,name,upload_key,invite_code,invited_by) VALUES($1,$2,$3,$4,$5,$6,$7) RETURNING *",
      &[&domain, &discord_id, &key, &name, &upload, &invite_code, &invited_by],
    )
    .await?;

  Ok(Users::from_row(row).unwrap())
}
