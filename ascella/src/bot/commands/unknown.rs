use crate::prelude::{BotResponse, Language};
use anyhow::Result;
use twilight_http::Client;
use twilight_model::application::interaction::ApplicationCommand;

pub async fn execute(_client: &Client, cmd: &ApplicationCommand) -> Result<BotResponse> {
  Ok(BotResponse::new().content(cmd.lang().await?.unknown_command()))
}
