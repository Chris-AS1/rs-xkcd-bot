use oxotly_bot::{configuration::BotSettings, utils::build_settings};

#[tokio::test]
async fn load_token_from_config() {
    let settings: BotSettings = build_settings();
    assert!(settings.token.len() > 0, "token is zero-length");
}
