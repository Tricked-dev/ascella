use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(PostgresMapper, Serialize, Deserialize, Debug, Clone)]
#[pg_mapper(table = "domains")]
pub struct Domains {
  pub domain: String,
  pub apex: bool,
  pub owner: i32,
}

#[derive(PostgresMapper, Serialize, Deserialize, Clone)]
#[pg_mapper(table = "codes")]
pub struct Codes {
  pub id: i32,
  pub claimed_by: Option<i32>,
  pub owner: i32,
  pub key: String,
}

#[derive(PostgresMapper, Serialize, Deserialize, Clone)]
#[pg_mapper(table = "embeds")]
pub struct Embeds {
  pub color: Option<String>,
  pub description: Option<String>,
  pub owner: i32,
  pub title: Option<String>,
  pub url: Option<String>,
}

#[derive(PostgresMapper, Serialize, Deserialize, Clone)]
#[pg_mapper(table = "images")]
pub struct Images {
  pub content_type: String,
  pub id: i32,
  pub owner: i32,
  pub redirect: Option<String>,
  pub vanity: String,
  pub public: Option<bool>,
}

#[derive(PostgresMapper, Serialize, Deserialize, Clone)]
#[pg_mapper(table = "pastes")]
pub struct Pastes {
  pub id: String,
  pub content: String,
}

#[derive(PostgresMapper, Serialize, Deserialize, Clone)]
#[pg_mapper(table = "users")]
pub struct Users {
  pub discord_id: String,
  pub domain: String,
  pub id: i32,
  pub key: String,
  pub name: String,
  pub autodelete: Option<i32>,
  pub emojis: Option<bool>,
}

// #[derive(PostgresMapper,Serialize, Deserialize, Clone)]
// #[pg_mapper(table = "gallery")]
// pub struct gallery {
// pub id: i32,
// pub public: Option<bool>,
// pub images: Int[],
// pub owner: i32
// }
