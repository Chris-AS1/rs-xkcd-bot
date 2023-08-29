use crate::configuration::Settings;
use crate::database::connect;
use crate::utils::{get_random_comic, rate_limit_wrapper};
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

// TODO: wrap get_random_comic into a function for rate limiting
async fn commands_handler(
    bot: Bot,
    settings: Settings,
    msg: Message,
    cmd: Command,
) -> Result<(), RequestError> {
    let tmp: String;
    let text: String = match cmd {
        Command::Help => format!("{}", Command::descriptions()),
        Command::Hello => {
            tmp = format!("hello @{}", msg.from().unwrap().username.as_ref().unwrap());
            tmp
        }
        Command::XKCD => match rate_limit_wrapper(
            get_random_comic,
            &mut connect().unwrap(),
            settings,
            msg.from().unwrap().username.as_ref().unwrap().to_string(),
        )
        .await
        {
            // Command::XKCD => match get_random_comic(settings).await {
            Ok(link) => {
                tmp = link;
                tmp
            }
            Err(e) => {
                log::error!("{:?}", e);
                "there has been an error".into()
            }
        },
    };

    bot.send_message(msg.chat.id, text).await?;
    Ok(())
}
