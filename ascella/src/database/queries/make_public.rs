use crate::database::queries::prelude::*;

pub async fn exec(id: i32, owner: i32) -> Result<()> {
  get_tokio_postgres()
    .await
    .query_one(
      "UPDATE images SET public = true WHERE owner = $1 AND id = $2 AND redirect IS NULL RETURNING id",
      &[&owner, &id],
    )
    .await?;
  Ok(())
}
