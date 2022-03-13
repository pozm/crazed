use crate::utils;
use serenity::client::Context;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;
#[command]
// Limit command usage to guilds.
#[only_in(guilds)]
#[aliases(av)]
pub async fn avatar(ctx: &Context, msg: &Message) -> CommandResult {
    let u = msg.mentions.get(0).unwrap_or(&msg.author);
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                utils::embed::default(e).image(
                    u.avatar_url()
                        .clone()
                        .unwrap_or(msg.author.default_avatar_url()),
                )
            })
        })
        .await?;
    Ok(())
}
