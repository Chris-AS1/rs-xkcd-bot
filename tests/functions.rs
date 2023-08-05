use oxotly_bot::{
    configuration::BotSettings,
    utils::{build_settings, get_random_comic},
};

#[tokio::test]
async fn obtain_comic_link() {
    let settings: BotSettings = build_settings();

    assert!(
        get_random_comic(settings).await.is_ok(),
        "a link to the comic hasn't been found"
    );
}
