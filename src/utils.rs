use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone, PartialEq, Debug)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "says hello")]
    Hello,
    #[command(description = "gets a random comic")]
    XKCD,
}
