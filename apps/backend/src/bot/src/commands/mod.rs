use super::prelude::*;

pub async fn builtin_exec(client: &Client, cmd: &ApplicationCommand) -> Result<()> {
    let user = get_user_discord::exec(
        cmd.member
            .as_ref()
            .unwrap()
            .user
            .as_ref()
            .unwrap()
            .id
            .to_string(),
    )
    .await;
    let is_owner = cmd
        .member
        .as_ref()
        .unwrap()
        .user
        .as_ref()
        .unwrap()
        .id
        .to_string()
        == "336465356304678913";
    let value = match (cmd.data.name.as_str(), user, is_owner) {
        ("eval", Ok(user), true) => eval::execute(client, cmd, user).await,
        ("codedrop", Ok(user), true) => codedrop::execute(client, cmd, user).await,

        ("codes", Ok(user), _) => codes::execute(client, cmd, user).await,
        ("delete_latest", Ok(user), _) => delete_latest::execute(client, cmd, user).await,
        ("delete", Ok(user), _) => delete::execute(client, cmd, user).await,
        ("domain", Ok(user), _) => domain::execute(client, cmd, user).await,
        ("embed", Ok(user), _) => embed::execute(client, cmd, user).await,
        ("emojis", Ok(user), _) => emojis::execute(client, cmd, user).await,
        ("funny_redirect", Ok(user), _) => funny_redirect::execute(client, cmd, user).await,
        ("profile", Ok(user), _) => profile::execute(client, cmd, user).await,
        ("redirect", Ok(user), _) => redirect::execute(client, cmd, user).await,

        ("domains", _, _) => domains::execute(client, cmd).await,
        ("stats", _, _) => stats::execute(client, cmd).await,
        ("help", _, _) => help::execute(client, cmd).await,
        ("redeem", _, _) => redeem::execute(client, cmd).await,
        ("unknown", _, _) => unknown::execute(client, cmd).await,
        ("user", _, _) => user::execute(client, cmd).await,
        _ => {
            client
                .interaction_callback(
                    cmd.id,
                    &cmd.token,
                    &ChannelMessageWithSource(CallbackData {
                        allowed_mentions: Some(AllowedMentions {
                            parse: vec![],
                            users: vec![],
                            roles: vec![],
                            replied_user: true,
                        }),
                        components: None,
                        content: Some(String::from("Not a user of the image uploader")),
                        embeds: vec![],
                        flags: Some(MessageFlags::EPHEMERAL),
                        tts: Some(false),
                    }),
                )
                .exec()
                .await?;
            Ok(())
        }
    };

    send_text_webhook(format!(
        "**{}**: {}",
        cmd.data.name.as_str(),
        cmd.member.as_ref().unwrap().user.as_ref().unwrap().name
    ))
    .await?;
    match value {
        Ok(_) => {}
        Err(err) => {
            send_text_webhook(format!(
                "**{}**: {}, ERROR OCCURRED {:?}",
                cmd.data.name.as_str(),
                cmd.member.as_ref().unwrap().user.as_ref().unwrap().name,
                err
            ))
            .await?;
            println!("-+ {:?}", err);
        }
    };
    Ok(())
}

pub mod adddomain;
pub mod codedrop;
pub mod codes;
pub mod delete;
pub mod delete_latest;
pub mod domain;
pub mod domains;
pub mod embed;
pub mod emojis;
pub mod eval;
pub mod funny_redirect;
pub mod help;
pub mod profile;
pub mod redeem;
pub mod redirect;
pub mod stats;
pub mod unknown;
pub mod user;
