use anyhow::Result;
use lazy_static::lazy_static;
use tokio::sync::OnceCell;
use tokio_postgres::{Client, NoTls};
pub mod queries;
pub mod structs;
lazy_static! {
    static ref POSTGRES: OnceCell<Client> = OnceCell::new();
}

pub async fn init_pg_tokio() -> Result<&'static Client> {
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    log::info!("CALLED INIT TOKIO");
    Ok(POSTGRES.get_or_init(|| async { client }).await)
}

pub async fn get_tokio_postgres() -> &'static Client {
    log::info!("get_tokio_postgres");
    if let Some(db) = POSTGRES.get() {
        db
    } else {
        return init_pg_tokio().await.unwrap();
    }
}
