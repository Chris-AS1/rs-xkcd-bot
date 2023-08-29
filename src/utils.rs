use crate::{
    commands,
    configuration::BotSettings,
    configuration::Settings,
    database::consume_daily,
    errors::{self, Error},
};
use anyhow::Context;
use dotenv::dotenv;
use html5ever::driver::{self, ParseOpts};
use reqwest::{self, header::USER_AGENT};
use scraper::{Html, Selector};
use std::{future::Future, sync::Once};
use teloxide::prelude::*;
use tendril::TendrilSink;

static INIT: Once = Once::new();

// initializes env variables, used for correct logging
pub fn setup() {
    INIT.call_once(|| {
        dotenv().ok();
        env_logger::init();
    });
}

pub fn build_settings() -> Result<Settings, errors::Error> {
    let settings = Settings::new()?;
    println!("loaded following settings: {:#?}", &settings);

    Ok(settings)
}

pub fn build_bot(
    settings: Settings,
) -> Dispatcher<Bot, teloxide::RequestError, teloxide::dispatching::DefaultKey> {
    let bot = Bot::new(&settings.bot.token);

    Dispatcher::builder(bot.clone(), commands::schema())
        // Here you specify initial dependencies that all handlers will receive; they can be
        // database connections, configurations, and other auxiliary arguments. It is similar to
        // `actix_web::Extensions`.
        .dependencies(dptree::deps![settings.clone()])
        .default_handler(|upd| async move {
            log::warn!("Unhandled update: {:?}", upd);
        })
        .error_handler(LoggingErrorHandler::with_custom_text(
            "An error has occurred in the dispatcher",
        ))
        .enable_ctrlc_handler()
        .build()
}

pub async fn spawn() {
    let settings = build_settings().unwrap();
    let mut bot = build_bot(settings);
    bot.dispatch().await;
}

pub async fn spawn_from_settings(settings: Settings) {
    let mut bot = build_bot(settings);
    bot.dispatch().await;
}

pub async fn rate_limit_wrapper<F, Fut>(
    f: F,
    con: &mut redis::Connection,
    settings: Settings,
    username: String,
) -> Result<std::string::String, errors::Error>
where
    F: FnOnce(BotSettings) -> Fut,
    Fut: Future<Output = Result<String, Error>>,
{
    if consume_daily(con, &settings, username).is_err() {
        return Err(Error::RateLimitError);
    }

    f(settings.bot).await
}

pub async fn get_random_comic(settings: BotSettings) -> Result<String, Error> {
    let client = reqwest::Client::new();

    let body = client
        .get(settings.xkcd_url)
        .header(USER_AGENT, settings.user_agent)
        .send()
        .await
        .context("request failed")?
        .text()
        .await
        .context("failed to extract text")?;

    let parser = driver::parse_document(Html::new_document(), ParseOpts::default());
    let html = parser.one(body);
    let selector = Selector::parse(r#"div[id="comic"] img"#).unwrap();
    let a = html.select(&selector);

    for element in a {
        let src: Vec<(&str, &str)> = element
            .value()
            .attrs()
            .filter(|(x, _)| *x == "src")
            .collect();

        if let Some((_, link)) = src.first() {
            return Ok(format!("https:{}", link).to_string());
        };
    }

    Err(errors::Error::LinkNotFoundError)
}
