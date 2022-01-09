use anyhow::Result;
use twilight_http::Client;
use twilight_model::application::interaction::ApplicationCommand;
use twilight_model::{
    application::callback::{CallbackData, InteractionResponse::ChannelMessageWithSource},
    channel::message::{AllowedMentions, MessageFlags},
};

pub async fn execute(client: &Client, cmd: &ApplicationCommand) -> Result<()> {
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
                content: Some(String::from(
                    "The command you tried running **doesn't** exist...",
                )),
                embeds: None,
                flags: Some(MessageFlags::EPHEMERAL),
                tts: Some(false),
            }),
        )
        .exec()
        .await?;

    Ok(())
}
