use crate::utils;
use serenity::client::Context;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::http::CacheHttp;
use serenity::model::channel::Message;
use serenity::prelude::Mentionable;

#[command]
// Limit command usage to guilds.
#[only_in(guilds)]
#[aliases(ui)]
pub async fn userinfo(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mem = msg
        .guild_id
        .unwrap()
        .member(&ctx.http, msg.mentions.get(0).unwrap_or(&msg.author))
        .await?;
    let highest_role = mem.highest_role_info(&ctx.cache).await.unwrap().0.to_role_cached(&ctx.cache).await.unwrap();
    let perms = mem.permissions(&ctx).await;
    let guild = msg.guild(&ctx.cache).await.expect("troll");
    let pres = guild.presences.get(&mem.user.id).ok_or("no presence");
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.reference_message(msg)
            .embed(|e| {
                utils::embed::default(e)
                    .thumbnail(
                        mem.avatar_url().clone().unwrap_or(
                            mem.user
                                .avatar_url()
                                .clone()
                                .unwrap_or(msg.author.default_avatar_url()),
                        ),
                    )
                    .field("tag", mem.user.tag(), false)
                    .field("id", mem.user.id, false)
                    .field("joined at", mem.joined_at.unwrap().to_string(), false)
                    .field("made at", mem.user.created_at().to_string(), false)
                    .field("highest role", highest_role.name, false)
                    .field(
                        "perm bits",
                        if perms.is_ok() {
                            perms.unwrap().to_string()
                        } else {
                            perms.unwrap_err().to_string()
                        },
                        false,
                    )
                    .field(
                        "devices lol",
                        if pres.is_ok() {
                            let pre = pres.unwrap();
                            let status = pre.clone().client_status.unwrap();
                            format!(
                                "mobile = {:#?}\ndesktop = {:#?}\nweb = {:#?}",
                                status.mobile, status.desktop, status.web
                            )
                        } else {
                            pres.unwrap_err().to_string()
                        },
                        false,
                    )
            })
        })
        .await?;
    Ok(())
}
