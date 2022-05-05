use anyhow::Result;
use twilight_http::Client;
use twilight_model::application::interaction::ApplicationCommand;

use crate::prelude::BotResponse;

pub async fn execute(_client: &Client, _cmd: &ApplicationCommand) -> Result<BotResponse> {
  Ok(BotResponse::new().content("The command you tried running **doesn't** exist..."))
}
