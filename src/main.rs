use dotenv::dotenv;
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting echo bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        match msg.text() {
            Some(text) => bot.send_message(msg.chat.id, text).await?,
            None => bot.send_message(msg.chat.id, "Send me plain text.").await?,
        };

        Ok(())
    })
    .await;
}
