use crate::commands::*;
use crate::prelude::*;

pub fn create_embed() -> EmbedBuilder {
    EmbedBuilder::new()
        .color(0x0e1b98)
        .footer(EmbedFooter{proxy_icon_url:None,
            text: "© Tricked".to_string(),
            icon_url:Some("https://cdn.discordapp.com/attachments/811240979918618634/877975737141960724/a_a4ecfac9730946b91da3d40b0490b30f.gif".to_string()) })
}

pub fn get_commands(domain_options: Vec<(String, String)>) -> [Command; 18] {
    [
        adddomain::command(),
        autodelete::command(),
        codedrop::command(),
        codes::command(),
        delete_latest::command(),
        delete::command(),
        domain::command(domain_options.clone()),
        domains::command(),
        embed::command(),
        emojis::command(),
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
