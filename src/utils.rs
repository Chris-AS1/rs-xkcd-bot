use crate::{configuration::BotInterface, errors};
use anyhow::Context;
use html5ever::driver::{self, ParseOpts};
use reqwest::{self, header::USER_AGENT};
use scraper::{Html, Selector};
use tendril::TendrilSink;

pub async fn get_random_comic(bot_interface: BotInterface) -> Result<String, errors::Error> {
    let client = reqwest::Client::new();

    let body = client
        .get(bot_interface.bot_settings.xkcd_url)
        .header(USER_AGENT, bot_interface.bot_settings.user_agent)
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
