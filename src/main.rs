use dotenv::dotenv;
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    log::info!("Starting Oxotly Bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        bot.send_dice(msg.chat.id).await?;
        Ok(())
    })
    .await;
}
