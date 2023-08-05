use html5ever::driver::{self, ParseOpts};
use oxotly_bot::configuration::BotInterface;
use reqwest::header::USER_AGENT;
use scraper::{Html, Selector};
use tendril::TendrilSink;

#[test]
fn parse_xkcd_page() {
    let client = reqwest::blocking::Client::new();
    let bot_interface = BotInterface::new().expect("failed to create BotInterface");

    let body = client
        .get(bot_interface.bot_settings.xkcd_url)
        .header(USER_AGENT, bot_interface.bot_settings.user_agent)
        .send()
        .unwrap()
        .text()
        .unwrap();

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
        }
    }

    assert!(false)
}
