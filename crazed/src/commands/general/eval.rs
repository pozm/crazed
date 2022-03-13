use crate::utils;
use serenity::client::Context;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;


pub async fn run_code(lang:&str, code:String ) -> String {
    let client =reqwest::Client::new();
    if let Ok(res) = client.post("http://exec:8000/api/run").header("Content-Type","text/plain").header("lang",lang).body(code).send().await {
        if let Ok(body) = res.text().await {
            return body;
        } else {
            String::from("unable to resolve body")
        }
    } else {
        String::from("Unable to connect to server")
    }
}

#[command]
#[only_in(guilds)]
#[aliases(ev)]
#[description("Evaluates code, allows for `rust`, `c++`, `c`, `js`, and `python` (py)\n You can also upload files after specifying the language")]
#[usage("<language> <code>")]
#[example("py print('hi') ")]
pub async fn eval(ctx: &Context, msg: &Message) -> CommandResult {
    let mut args = msg.content.split_whitespace();
    let lang = args.nth(1).unwrap_or("");
    println!("{}",lang);
    if lang.len() < 1 {
        msg.reply(ctx, "Please specify a language.").await;
        return Ok(());
    }
    if let Some(f) = msg.attachments.first() {
        let code = f.download().await.unwrap_or(vec![]);
        msg.reply(ctx, run_code(lang, String::from_utf8(code).unwrap_or(String::new()) ).await).await;
    } else {
        let code = args.collect::<Vec<&str>>().join(" ");
        if code.len() < 1 {
            msg.reply(ctx, "Please specify the code.").await;
            return Ok(());
        } else {
            msg.reply(ctx, run_code(lang, code).await).await;
        }
    };
    Ok(())
}
