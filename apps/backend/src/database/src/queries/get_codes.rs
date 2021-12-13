use crate::queries::prelude::*;

#[cached(size = 100, time = 120, result = true)]
pub async fn exec(id: i32) -> Result<Vec<Codes>> {
    let row = get_tokio_postgres()
        .await
        .query("SELECT * FROM codes WHERE owner = $1", &[&id])
        .await?;

    Ok(row
        .iter()
        .map(|row| Codes::from_row_ref(row).unwrap())
        .collect::<Vec<_>>())
}
