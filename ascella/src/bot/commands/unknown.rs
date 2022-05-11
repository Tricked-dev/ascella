use crate::prelude::{AscellaLanguage, BotResponse};
use anyhow::Result;
use twilight_http::Client;
use twilight_model::application::interaction::ApplicationCommand;

pub async fn execute(_client: &Client, cmd: &ApplicationCommand) -> Result<BotResponse> {
  Ok(BotResponse::wcontent(cmd.lang().await?.unknown_command()))
}
