use anyhow::Result;
use twilight_http::Client;
use twilight_model::application::interaction::ApplicationCommand;
use twilight_model::channel::message::{AllowedMentions, MessageFlags};

use crate::prelude::BotResponse;

pub async fn execute(client: &Client, cmd: &ApplicationCommand) -> Result<BotResponse> {
  Ok(BotResponse::new().content("The command you tried running **doesn't** exist..."))
}
