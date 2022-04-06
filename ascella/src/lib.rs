pub mod bot;
pub mod database;
pub mod http;
pub mod ratelimit;
pub mod util;

pub mod prelude {
  pub use crate::database::queries::*;
  pub use crate::database::structs::*;
  pub use crate::database::*;
  pub use crate::util::{create_config, ran_str, random_emojis, send_text_webhook, upload_success, Error, SendMessage};
  pub use actix_multipart::Multipart;
  pub use actix_web::{HttpRequest, HttpResponse};
  pub use anyhow::anyhow;
  pub use anyhow::Result;
  pub use byte_unit::Byte;
  pub use cached::{proc_macro::cached, Cached, CachedAsync};
  pub use futures::{StreamExt, TryStreamExt};
  pub use paperclip::actix::api_v2_operation;
  pub use paperclip::actix::*;
  pub use serde::{Deserialize, Serialize};
  pub use serde_json::{from_str, json, Value};
  pub use std::{
    collections::HashMap,
    fs::{metadata, read},
    io::{Cursor, Write},
  };
  pub use tokio::fs::create_dir_all;
  pub use tokio_pg_mapper::FromTokioPostgresRow;
  lazy_static! {
    pub static ref START_TIME: OnceCell<Instant> = OnceCell::new();
  }

  pub use crate::apply_responders;
  pub use crate::bot::utils::*;
  pub use crate::database::queries::*;
  pub use crate::database::structs::*;
  pub use crate::database::*;
  pub use crate::util::*;
  pub use actix_web::body::BoxBody;
  pub use actix_web::Responder;
  pub use futures::future;
  pub use humantime::format_duration;
  pub use lazy_static::lazy_static;
  use once_cell::sync::OnceCell;
  pub use rand::{prelude::SliceRandom, Rng};
  pub use std::env;
  pub use std::slice::Iter;
  pub use std::{
    fs, io,
    path::PathBuf,
    process::{self, Command as ProcessCommand},
    time::Instant,
  };
  pub use twilight_embed_builder::{EmbedBuilder, EmbedFieldBuilder};
  pub use twilight_gateway::{cluster::ShardScheme, Cluster, Event, Intents};
  pub use twilight_http::Client;
  pub use twilight_model::datetime::Timestamp;
  pub use twilight_model::{
    application::{
      callback::{CallbackData, InteractionResponse::ChannelMessageWithSource},
      command::{Command, CommandType},
      interaction::{
        application_command::{CommandDataOption, CommandOptionValue},
        ApplicationCommand, Interaction,
      },
    },
    channel::{
      embed::EmbedFooter,
      message::{AllowedMentions, MessageFlags},
    },
    id::*,
  };
  pub use twilight_util::builder::command::*;
}
