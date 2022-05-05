use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::prelude::*;

pub async fn builtin_exec(client: &Client, cmd: &ApplicationCommand) -> Result<()> {
  let user = get_user_discord::exec(cmd.member.as_ref().unwrap().user.as_ref().unwrap().id.to_string()).await;
  let is_owner = cmd.member.as_ref().unwrap().user.as_ref().unwrap().id.to_string() == "336465356304678913";
  let value = match (cmd.data.name.as_str(), user, is_owner) {
    ("eval", Ok(user), true) => eval::execute(client, cmd, user).await,

    ("autodelete", Ok(user), _) => autodelete::execute(client, cmd, user).await,
    ("delete_latest", Ok(user), _) => delete_latest::execute(client, cmd, user).await,
    ("delete", Ok(user), _) => delete::execute(client, cmd, user).await,
    ("domain", Ok(user), _) => domain::execute(client, cmd, user).await,
    ("embed", Ok(user), _) => embed::execute(client, cmd, user).await,
    ("url_style", Ok(user), _) => url_style::execute(client, cmd, user).await,
    ("funny_redirect", Ok(user), _) => funny_redirect::execute(client, cmd, user).await,
    ("profile", Ok(user), _) => profile::execute(client, cmd, user).await,
    ("redirect", Ok(user), _) => redirect::execute(client, cmd, user).await,

    ("domains", _, _) => domains::execute(client, cmd).await,
    ("stats", _, _) => stats::execute(client, cmd).await,
    ("help", _, _) => help::execute(client, cmd).await,
    ("redeem", _, _) => redeem::execute(client, cmd).await,
    ("unknown", _, _) => unknown::execute(client, cmd).await,
    ("user", _, _) => user::execute(client, cmd).await,
    _ => Ok(BotResponse::new().content("Not a user of the image uploader").private()),
  };

  send_text_webhook(format!("**{}**: {}", cmd.data.name.as_str(), cmd.member.as_ref().unwrap().user.as_ref().unwrap().name)).await?;
  match value {
    Ok(value) => {
      let mut response = InteractionResponseDataBuilder::new();
      if let Some(content) = value.get_content() {
        response = response.content(content.to_string());
      }
      if let Some(embed) = value.get_embed() {
        response = response.embeds([embed.to_owned()]);
      }
      if value.is_private() {
        response = response.flags(MessageFlags::EPHEMERAL);
      }
      client
        .interaction(Id::new(env::var("APPLICATION_ID").unwrap().parse::<u64>().unwrap()))
        .create_response(
          cmd.id,
          &cmd.token,
          &InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(response.build()),
          },
        )
        .exec()
        .await?;
    }
    Err(err) => {
      send_text_webhook(format!(
        "**{}**: {}, ERROR OCCURRED {:?}",
        cmd.data.name.as_str(),
        cmd.member.as_ref().unwrap().user.as_ref().unwrap().name,
        err
      ))
      .await?;
      println!("-+ {:?}", err);
      log::error!("-+ {:?}", err);
    }
  };
  Ok(())
}

pub mod adddomain;
pub mod autodelete;
pub mod delete;
pub mod delete_latest;
pub mod domain;
pub mod domains;
pub mod embed;
pub mod eval;
pub mod funny_redirect;
pub mod help;
pub mod profile;
pub mod redeem;
pub mod redirect;
pub mod stats;
pub mod unknown;
pub mod url_style;
pub mod user;
