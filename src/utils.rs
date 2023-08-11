use crate::{commands, configuration::BotSettings, configuration::Settings, errors};
use anyhow::Context;
use html5ever::driver::{self, ParseOpts};
use reqwest::{self, header::USER_AGENT};
use scraper::{Html, Selector};
use teloxide::prelude::*;
use tendril::TendrilSink;

pub fn build_settings() -> Result<Settings, errors::Error> {
    // let settings = BotSettings::new().expect("failed on creating a BotSettings instance");
    let settings = Settings::new()?;
    println!("loaded following settings: {:#?}", &settings);

    Ok(settings)
}

pub fn build_bot(
    settings: BotSettings,
) -> Dispatcher<Bot, teloxide::RequestError, teloxide::dispatching::DefaultKey> {
    let bot = Bot::new(&settings.token);

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
    let mut bot = build_bot(settings.bot);
    bot.dispatch().await;
}

pub async fn spawn_from_settings(settings: Settings) {
    let mut bot = build_bot(settings.bot);
    bot.dispatch().await;
}

pub async fn get_random_comic(settings: BotSettings) -> Result<String, errors::Error> {
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
        println!("{:#?}", element.value());
        let src: Vec<(&str, &str)> = element
            .value()
            .attrs()
            .filter(|(x, _)| *x == "src")
            .collect();

        if let Some((_, link)) = src.first() {
            println!("{:#?}", link);
            return Ok(format!("https:{}", link).to_string());
        };
    }

    Err(errors::Error::LinkNotFoundError)
}
