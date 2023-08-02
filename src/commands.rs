use crate::configuration::BotInterface;
use crate::utils::get_random_comic;
use teloxide::dispatching::DpHandlerDescription;
use teloxide::utils::command::BotCommands;
use teloxide::{prelude::*, RequestError};

#[derive(BotCommands, Clone, PartialEq, Debug)]
#[command(
    rename_rule = "lowercase",
    description = "These are the supported commands:"
)]

pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "says hello")]
    Hello,
    #[command(description = "gets a random comic")]
    XKCD,
}

// https://github.com/teloxide/teloxide/blob/master/crates/teloxide/examples/dispatching_features.rs#L32
pub fn schema() -> Handler<'static, DependencyMap, Result<(), RequestError>, DpHandlerDescription> {
    let handler = Update::filter_message()
        // You can use branching to define multiple ways in which an update will be handled. If the
        // first branch fails, an update will be passed to the second branch, and so on.
        .branch(
            dptree::entry()
                // Filter commands: the next handlers will receive a parsed `SimpleCommand`.
                .filter_command::<Command>()
                // If a command parsing fails, this handler will not be executed.
                .endpoint(commands_handler),
        );
    handler
}

async fn commands_handler(
    bot_interface: BotInterface,
    bot: Bot,
    msg: Message,
    cmd: Command,
) -> Result<(), RequestError> {
    let tmp;
    let text = match cmd {
        Command::Help => "help",
        Command::Hello => {
            tmp = format!("hello @{}", msg.from().unwrap().username.as_ref().unwrap());
            tmp.as_str()
        }
        Command::XKCD => get_random_comic(bot_interface),
    };

    bot.send_message(msg.chat.id, text).await?;
    Ok(())
}
