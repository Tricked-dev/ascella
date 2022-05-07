use crate::Lang;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

use crate::prelude::*;

#[derive(PostgresMapper, Serialize, Deserialize, Debug, Clone, TS)]
#[pg_mapper(table = "domains")]
#[ts(export)]
pub struct Domains {
  pub domain: String,
  pub apex: bool,
  pub owner: i32,
}

#[derive(PostgresMapper, Serialize, Deserialize, Clone, Apiv2Schema, TS)]
#[pg_mapper(table = "embeds")]
#[ts(export)]
pub struct Embeds {
  pub color: Option<String>,
  pub description: Option<String>,
  pub owner: i32,
  pub title: Option<String>,
  pub url: Option<String>,
}

#[derive(PostgresMapper, Serialize, Deserialize, Clone, TS)]
#[pg_mapper(table = "images")]
#[ts(export)]
pub struct Images {
  pub content_type: String,
  pub id: i32,
  pub owner: i32,
  pub redirect: Option<String>,
  pub vanity: String,
  pub views: i32,
}

#[derive(PostgresMapper, Serialize, Deserialize, Clone, Apiv2Schema, TS)]
#[pg_mapper(table = "images")]
#[ts(export)]
pub struct SimpleImages {
  pub id: i32,
  pub vanity: String,
}

#[derive(PostgresMapper, Serialize, Deserialize, Clone, TS)]
#[pg_mapper(table = "pastes")]
#[ts(export)]
pub struct Pastes {
  pub id: String,
  pub content: String,
}

#[derive(PostgresMapper, Serialize, Deserialize, Clone, Apiv2Schema, TS)]
#[pg_mapper(table = "users")]
#[ts(export)]
pub struct Users {
  pub discord_id: String,
  pub domain: String,
  pub id: i32,
  pub key: String,
  pub name: String,
  pub autodelete: Option<i32>,
  pub deleteall: Option<i32>,
  pub upload_key: Option<String>,
  pub url_style: i32,
  pub invite_code: Option<String>,
  pub invited_by: i32,
  pub lang: String,
}

pub fn locale_to_lang(locale: &str) -> Lang {
  match locale {
    "en" => Lang::En,
    "pl" => Lang::Pl,
    "nl" => Lang::Nl,
    "es" => Lang::Es,
    "fr" => Lang::Fr,
    "bg" => Lang::Bg,
    _ => Lang::En,
  }
}

impl Users {
  pub fn lang(&self) -> Lang {
    locale_to_lang(self.lang.as_str())
  }
  pub fn set_lang<T: ToString>(&mut self, lang: T) {
    self.lang = lang.to_string();
  }
}
