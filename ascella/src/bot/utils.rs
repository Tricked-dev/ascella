use twilight_model::channel::embed::Embed;
use twilight_util::builder::embed::EmbedBuilder;

use crate::bot::commands::*;
use crate::prelude::*;

pub fn create_embed() -> EmbedBuilder {
  EmbedBuilder::new().color(0x0e1b98).footer(EmbedFooter {
    proxy_icon_url: None,
    text: "© Tricked".to_string(),
    icon_url: Some("https://cdn.discordapp.com/attachments/811240979918618634/877975737141960724/a_a4ecfac9730946b91da3d40b0490b30f.gif".to_string()),
  })
}
use crate::Lang;
use async_trait::async_trait;

#[async_trait]
pub trait Language {
  async fn lang(&self) -> Result<Lang>;
}

#[async_trait]
impl Language for ApplicationCommand {
  async fn lang(&self) -> Result<Lang> {
    let self_clone = self.clone();
    if let Some(member) = self_clone.member.as_ref() {
      if let Some(user) = member.user.as_ref() {
        if let Ok(user) = get_user_discord::exec(user.id.to_string()).await {
          return Ok(user.lang());
        }
      }
    };
    Ok(Lang::En)
  }
}

pub fn get_commands(domain_options: Vec<(String, String)>) -> [Command; 17] {
  [
    adddomain::command(),
    autodelete::command(),
    delete_latest::command(),
    delete::command(),
    domain::command(domain_options.clone()),
    domains::command(),
    embed::command(),
    url_style::command(),
    eval::command(),
    funny_redirect::command(domain_options.clone()),
    help::command(),
    lang::command(),
    profile::command(),
    redeem::command(),
    redirect::command(domain_options),
    stats::command(),
    user::command(),
  ]
}

pub fn get_arg(mut args: Iter<CommandDataOption>, key: &str) -> Option<String> {
  let domain = args.find(|e| e.name == key);
  if let Some(domain) = domain {
    match &domain.value {
      CommandOptionValue::String(val) => Some(val.clone()),
      _ => None,
    }
  } else {
    None
  }
}
pub fn get_arg_int(mut args: Iter<CommandDataOption>, key: &str) -> Option<i64> {
  let domain = args.find(|e| e.name == key);
  if let Some(domain) = domain {
    match &domain.value {
      CommandOptionValue::Integer(val) => Some(*val),
      _ => None,
    }
  } else {
    None
  }
}
pub fn get_arg_default(args: Iter<CommandDataOption>, key: &str, default: &str) -> String {
  get_arg(args, key).unwrap_or_else(|| default.to_owned())
}

pub fn format_profile(user: &Users, invites: i64, images: i64, upload_key: Option<String>) -> String {
  let key = if let Some(key) = upload_key { key } else { user.upload_key.as_ref().unwrap().to_string() };
  format!(
    r"dashboard: https://dash.ascella.host
id: `{id}`
name: `{name}`
discord id: `{discord}`
password: `{pass}`
invite_code: `{invite}`
domain: `{domain}`
images: `{images}`
users invited: `{invites}`
download config [here](https://ascella.wtf/v2/ascella/config?auth={upload_key})
```json
{config}
```",
    id = user.id,
    name = user.name,
    discord = user.discord_id,
    pass = user.key,
    domain = user.domain,
    invite = user.invite_code.as_ref().unwrap_or(&"none".to_owned()),
    config = serde_json::to_string_pretty(&create_config(&key)).unwrap(),
    images = images,
    upload_key = key
  )
}
#[derive(Clone, Debug)]
pub struct BotResponse {
  content: Option<String>,
  embed: Option<Embed>,
  private: bool,
}
impl BotResponse {
  pub fn new() -> Self {
    Self {
      content: None,
      embed: None,
      private: false,
    }
  }
  pub fn content<T: Into<String>>(mut self, content: T) -> Self {
    self.content = Some(content.into());
    self
  }
  pub fn embed(mut self, embed: Embed) -> Self {
    self.embed = Some(embed);
    self
  }
  pub fn private(mut self) -> Self {
    self.private = true;
    self
  }
  pub fn get_content(&self) -> Option<&str> {
    self.content.as_deref()
  }
  pub fn get_embed(&self) -> Option<&Embed> {
    self.embed.as_ref()
  }
  pub fn is_private(&self) -> bool {
    self.private
  }
}
