use oxotly_bot::{
    configuration::Settings,
    utils::{build_settings, get_random_comic},
};

#[ignore]
#[tokio::test]
async fn obtain_comic_link() {
    let settings: Settings = build_settings().unwrap();

    assert!(
        get_random_comic(settings.bot).await.is_ok(),
        "a link to the comic hasn't been found"
    );
}
