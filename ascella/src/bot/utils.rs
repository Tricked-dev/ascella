use crate::bot::commands::*;
use crate::prelude::*;

pub fn create_embed() -> EmbedBuilder {
  EmbedBuilder::new().color(0x0e1b98).footer(EmbedFooter {
    proxy_icon_url: None,
    text: "© Tricked".to_string(),
    icon_url: Some("https://cdn.discordapp.com/attachments/811240979918618634/877975737141960724/a_a4ecfac9730946b91da3d40b0490b30f.gif".to_string()),
  })
}

pub fn get_commands(domain_options: Vec<(String, String)>) -> [Command; 16] {
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

pub fn format_profile(user: &Users, images: Option<i64>, upload_key: Option<String>) -> String {
  format!(
    r"dashboard: https://dash.ascella.host
id: `{id}`
name: `{name}`
discord id: `{discord}`
password: `{pass}`
invite_code: `{invite}`
domain: `{domain}`
images: `{images}`

download config [here](https://ascella.wtf/v2/ascella/config?auth={upload_key})
```json
{config}
```",
    id = user.id,
    name = user.name,
    discord = user.discord_id,
    pass = user.key,
    domain = user.domain,
    invite = user.invite_code,
    config = serde_json::to_string_pretty(&create_config(&user.upload_key.as_ref().unwrap())).unwrap(),
    images = images.unwrap_or(0),
    upload_key = if let Some(key) = upload_key { key } else { user.upload_key.as_ref().unwrap().to_string() }
  )
}
