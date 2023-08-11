use oxotly_bot::{configuration::Environment, utils::build_settings};
use std::env;
use strum::IntoEnumIterator;

#[tokio::test]
async fn token_is_not_empty() {
    let settings = build_settings().unwrap();
    assert!(settings.bot.token.len() > 0, "token is zero-length")
}

#[test]
fn load_configs_in_all_environments() {
    for environment in Environment::iter() {
        env::set_var("APP_ENV", environment.to_string());
        build_settings().unwrap();
    }
}

#[ignore]
#[test]
fn token_overwritten_by_env() {
    let settings = build_settings().unwrap();
    assert_eq!("asd", settings.bot.token);
}
