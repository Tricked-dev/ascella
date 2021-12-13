use crate::queries::prelude::*;

#[cached(size = 100, time = 120, result = true)]
pub async fn exec(vanity: String) -> Result<Images> {
    let row = get_tokio_postgres()
        .await
        .query_one("SELECT * FROM images WHERE vanity = $1 LIMIT 1", &[&vanity])
        .await?;

    Ok(Images::from_row(row).unwrap())
}
