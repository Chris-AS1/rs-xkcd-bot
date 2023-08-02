use oxotly_bot::configuration::BotInterface;

#[tokio::test]
async fn create_bot_from_token() {
    let bot_interface = BotInterface::new().expect("failed to create BotInterface");
    let bot = bot_interface.bot;

    // println!("{:?}", bot);
    assert_eq!(bot.token(), bot_interface.bot_settings.token)
}
