use serenity::client::Context;
use serenity::framework::standard::{macros::hook, CommandResult, DispatchError};
use serenity::model::channel::Message;

#[hook]
pub async fn before(ctx: &Context, msg: &Message, command_name: &str) -> bool {
    println!(
        "Got command '{}' by user '{}'",
        command_name, msg.author.name
    );

    std::fs::create_dir_all(format!("./operations/{}",msg.id) );

    true // if `before` returns false, command processing doesn't happen.
}

#[hook]
pub async fn after(
    ctx: &Context,
    msg: &Message,
    command_name: &str,
    command_result: CommandResult,
) {
    std::fs::remove_dir_all(format!("./operations/{}",msg.id));
    match command_result {
        Ok(()) => println!("Processed command '{}'", command_name),
        Err(why) => {
            println!("Command '{}' returned error {:?}", command_name, why);
            msg.channel_id.say(&ctx.http,format!("command errored : {:?}",why)).await;
        }
    }
}

#[hook]
pub async fn unknown_command(_ctx: &Context, _msg: &Message, unknown_command_name: &str) {
    println!("Could not find command named '{}'", unknown_command_name);
}

#[hook]
pub async fn normal_message(_ctx: &Context, msg: &Message) {
    // println!("Message is not a command '{}'", msg.content);
    // if msg.channel(&ctx.cache).await.expect("HELP ME ").category().expect("CATEGORY").id == 879005649374150748 {
    //
    // }
}

#[hook]
pub async fn delay_action(ctx: &Context, msg: &Message) {
    // You may want to handle a Discord rate limit if this fails.
    let _ = msg.react(ctx, '⏱').await;
}

#[hook]
pub async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError) {
    if let DispatchError::Ratelimited(info) = error {
        // We notify them only once.
        if info.is_first_try {
            let _ = msg
                .channel_id
                .say(
                    &ctx.http,
                    &format!("Try this again in {} seconds.", info.as_secs()),
                )
                .await;
        }
    }
}
