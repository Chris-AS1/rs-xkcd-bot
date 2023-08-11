use oxotly_bot::utils::build_settings;

#[tokio::test]
async fn load_configs_and_token() {
    let settings = build_settings().unwrap();

    assert!(settings.bot.token.len() > 0, "token is zero-length")
}
