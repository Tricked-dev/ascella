use crate::queries::prelude::*;

#[cached(size = 100, time = 120, result = true)]
pub async fn exec() -> Result<Vec<Domains>> {
    let row = get_tokio_postgres()
        .await
        .query("SELECT * FROM domains", &[])
        .await?;
    let mut new_rows: Vec<Domains> = vec![];
    for row in row.iter() {
        new_rows.push(Domains::from_row_ref(row).unwrap())
    }
    Ok(new_rows)
}
