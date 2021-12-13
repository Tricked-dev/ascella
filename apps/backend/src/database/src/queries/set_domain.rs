use crate::queries::prelude::*;
pub async fn exec(id: i32, domain: String) -> Result<()> {
    get_tokio_postgres()
        .await
        .query(
            "UPDATE users SET domain = $1 WHERE id = $2",
            &[&domain, &id],
        )
        .await?;
    Ok(())
}
