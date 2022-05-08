#![feature(fmt_internals)]
#![feature(test)]

pub mod bot;
pub mod cron;
pub mod database;
pub mod http;
pub mod ratelimit;
pub mod util;

pub mod prelude {
  lazy_static! {
    pub static ref CLIENT: reqwest::Client = reqwest::Client::new();
    pub static ref START_TIME: OnceCell<Instant> = OnceCell::new();
    pub static ref HTTP: OnceCell<Arc<Client>> = OnceCell::new();
    pub static ref REVIEWS: OnceCell<Vec<Comment>> = OnceCell::new();
  }

  pub use super::Lang;
  pub use crate::bot::bot::Comment;
  pub use crate::bot::utils::*;
  pub use crate::database::queries::*;
  pub use crate::database::structs::*;
  pub use crate::database::*;
  pub use crate::util::*;
  pub use actix_multipart::Multipart;
  pub use actix_web::body::BoxBody;
  pub use actix_web::Responder;
  pub use actix_web::{HttpRequest, HttpResponse};
  pub use anyhow::anyhow;
  pub use anyhow::Result;
  pub use byte_unit::Byte;
  pub use cached::{proc_macro::cached, Cached, CachedAsync};
  pub use futures::{future, StreamExt, TryStreamExt};
  pub use humantime::format_duration;
  pub use lazy_static::lazy_static;
  pub use once_cell::sync::OnceCell;
  pub use paperclip::actix::*;
  pub use rand::{prelude::SliceRandom, Rng};
  pub use serde::{Deserialize, Serialize};
  pub use serde_json::{from_str, json, Value};
  pub use std::collections::HashMap;
  pub use std::env;
  pub use std::fs::{metadata, read};
  pub use std::io::{Cursor, Write};
  pub use std::path::PathBuf;
  pub use std::process::{self, Command as ProcessCommand};
  pub use std::slice::Iter;
  pub use std::sync::Arc;
  pub use std::{fs, io, time::Instant};
  pub use tokio::fs::create_dir_all;
  pub use tokio_pg_mapper::FromTokioPostgresRow;
  pub use ts_rs::TS;
  pub use twilight_gateway::{cluster::ShardScheme, Cluster, Event, Intents};
  pub use twilight_http::Client;
  pub use twilight_model::application::command::{Command, CommandType};
  pub use twilight_model::application::interaction::application_command::{CommandDataOption, CommandOptionValue};
  pub use twilight_model::application::interaction::{ApplicationCommand, Interaction};
  pub use twilight_model::channel::embed::EmbedFooter;
  pub use twilight_model::channel::message::{AllowedMentions, MessageFlags};
  pub use twilight_model::datetime::Timestamp;
  pub use twilight_model::id::*;
  pub use twilight_util::builder::command::*;
}

rosetta_i18n::include_translations!();
