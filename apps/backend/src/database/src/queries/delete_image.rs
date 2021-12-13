use crate::queries::prelude::*;
pub async fn exec(id: i32) -> Result<()> {
    get_tokio_postgres()
        .await
        .query("DELETE FROM images WHERE id = $1", &[&id])
        .await?;

    Ok(())
}
