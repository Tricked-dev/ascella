use super::prelude::*;

pub async fn exec() -> Result<Vec<Users>> {
  let rows = get_tokio_postgres()
    .await
    .query("SELECT * FROM users", &[])
    .await?
    .iter()
    .map(|x| Users::from_row_ref(x).unwrap())
    .collect::<Vec<Users>>();

  Ok(rows)
}
