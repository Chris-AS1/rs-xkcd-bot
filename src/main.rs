use dotenv::dotenv;
use oxotly_bot::{configuration::load_configuration, utils::Command};
use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;

async fn hello_command(bot: Bot, msg: Message) -> Result<Message, teloxide::RequestError> {
    bot.send_message(
        msg.chat.id,
        format!("hello @{}", msg.from().unwrap().username.as_ref().unwrap()),
    )
    .await
}

// TODO check if valid for images
async fn get_comic(bot: Bot, msg: Message) -> Result<Message, teloxide::RequestError> {
    unimplemented!()
}

async fn answer_handler(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Hello => hello_command(bot, msg).await?,
        Command::XKCD => get_comic(bot, msg).await?,
    };

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    let settings = load_configuration().expect("failed to load configs");

    println!("{:?}", settings);

    log::info!("Starting Oxotly Bot...");

    let bot = Bot::new(settings.token);
    Command::repl(bot, answer_handler).await;
}
