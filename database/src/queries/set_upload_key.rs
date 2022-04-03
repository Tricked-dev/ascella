use crate::queries::prelude::*;
pub async fn exec(id: i32, val: &str) -> Result<()> {
    get_tokio_postgres()
        .await
        .query(
            "UPDATE users SET upload_key = $1 WHERE id = $2",
            &[&val, &id],
        )
        .await?;
    Ok(())
}
