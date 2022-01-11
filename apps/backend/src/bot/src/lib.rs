pub mod bot;
pub mod commands;
pub mod utils;
pub use bot::start_bot;
pub mod prelude {
  lazy_static! {
    pub static ref START_TIME: OnceCell<Instant> = OnceCell::new();
  }

  pub use crate::utils::*;
  pub use anyhow::Result;
  pub use ascella_database::queries::*;
  pub use ascella_database::structs::*;
  pub use ascella_database::*;
  pub use ascella_util::*;
  pub use byte_unit::Byte;
  pub use cached::{proc_macro::cached, Cached, CachedAsync};
  pub use futures::future;
  pub use futures::StreamExt;
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
