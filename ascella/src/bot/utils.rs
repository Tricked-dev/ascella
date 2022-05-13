use strum::IntoEnumIterator;
use twilight_model::channel::embed::Embed;
use twilight_util::builder::embed::EmbedBuilder;

use crate::bot::commands::*;
use crate::prelude::*;

pub fn create_embed() -> EmbedBuilder {
  EmbedBuilder::new().color(0x0e1b98).footer(EmbedFooter {
    proxy_icon_url: None,
    text: "© Ascella".to_string(),
    icon_url: Some("https://cdn.discordapp.com/attachments/748957504666599507/974777620422074468/logo_but_png.png".to_string()),
  })
}
use crate::Lang;
use async_trait::async_trait;

#[async_trait]
pub trait AscellaLanguage {
  async fn lang(&self) -> Result<Lang>;
}

#[async_trait]
impl AscellaLanguage for ApplicationCommand {
  async fn lang(&self) -> Result<Lang> {
    let self_clone = self.clone();
    if let Some(member) = self_clone.member.as_ref() {
      if let Some(user) = member.user.as_ref() {
        if let Ok(user) = get_user_discord::exec(user.id.to_string()).await {
          return Ok(user.lang());
        } else if let Some(locale) = &user.locale {
          return Ok(locale_to_lang(locale));
        }
      }
    };
    Ok(Lang::En)
  }
}

pub trait Localize {
  fn localize(self) -> Self
  where
    Self: std::marker::Sized,
  {
    self
  }
}
impl Localize for CommandBuilder {
  fn localize(self) -> Self {
    let name = self.clone().build().name;
    let mut names = HashMap::new();
    let mut description = HashMap::new();
    for lang in Lang::iter() {
      if lang != Lang::Nl {
        continue;
      }
      let lang_code = format!("{:?}", lang).to_lowercase().replace("en", "en-US").replace("es", "es-ES").replace("pt", "pt-BR");
      names.insert(lang_code.clone(), lang.from_str(format!("{name}_name").as_ref()).expect("NOT LOCALIZED").to_owned().to_lowercase());
      description.insert(lang_code, lang.from_str(format!("{name}_desc").as_ref()).unwrap().to_owned());
    }
    self.name_localizations(names).description_localizations(description)
  }
}

#[test]
fn test_localize_shit() {
  for lang in Lang::iter() {
    println!("{}", format!("{:?}", lang).to_lowercase())
  }
}

pub fn get_commands(domain_options: Vec<(String, String)>) -> [Command; 18] {
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
    addbadge::command(),
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

mod emojis {
  pub const OG: &str = "<:OG:974427347065397289>";
  pub const SPONSOR: &str = "<:Donator:974427347048615976>";
  pub const TRANSLATOR: &str = "<:Translator:974427346989899866>";
  pub const CONTRIBUTOR: &str = "<:Contributor:974427346977316884>";
}

pub fn get_flags_str(user: &Users) -> String {
  if let Some(flags) = BadgeFlags::from_bits(user.flags as u32) {
    let mut res = String::new();
    if flags.contains(BadgeFlags::CONTRIBUTOR) {
      res.push_str(emojis::CONTRIBUTOR)
    }
    if flags.contains(BadgeFlags::SPONSOR) {
      res.push_str(emojis::SPONSOR)
    }
    if flags.contains(BadgeFlags::TRANSLATOR) {
      res.push_str(emojis::TRANSLATOR)
    }
    if flags.contains(BadgeFlags::OG) {
      res.push_str(emojis::OG)
    }
    res
  } else {
    String::new()
  }
}

pub fn format_profile(user: &Users, invites: i64, images: i64, upload_key: Option<String>) -> String {
  let key = if let Some(key) = upload_key { key } else { user.upload_key.as_ref().unwrap().to_string() };
  let flags = get_flags_str(user);
  format!(
    r"{flags}
dashboard: https://dash.ascella.host
id: `{id}`
name: `{name}`
discord id: `{discord}`
password: `{pass}`
invite_code: `{invite}`
domain: `{domain}`
images: `{images}`
users invited: `{invites}`
download config [here](https://ascella.wtf/v2/ascella/config?auth={key})
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
  )
  .trim()
  .to_owned()
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
  pub fn wembed(embed: Embed) -> Self {
    Self {
      content: None,
      embed: Some(embed),
      private: false,
    }
  }
  pub fn wcontent<T: Into<String>>(content: T) -> Self {
    Self {
      content: Some(content.into()),
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

use bitflags::bitflags;

bitflags! {
    pub struct BadgeFlags: u32 {
        const CONTRIBUTOR = 1 << 1;
        const TRANSLATOR = 1 << 2;
        const SPONSOR = 1 << 3;
        const OG = 1 << 4;
    }
}

#[test]
fn test_shit() {
  let flags = BadgeFlags::SPONSOR | BadgeFlags::TRANSLATOR;
  println!("{}", flags.bits())
}
