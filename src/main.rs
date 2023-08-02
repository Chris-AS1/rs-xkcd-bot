use dotenv::dotenv;
use oxotly_bot::commands;
use oxotly_bot::configuration::BotInterface;
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    log::info!("Starting Oxotly Bot...");

    let bot_interface = BotInterface::new().expect("failed to create BotInterface");

    println!("{:?}", &bot_interface);

    let bot = &bot_interface.bot;
    Dispatcher::builder(bot.clone(), commands::schema())
        // Here you specify initial dependencies that all handlers will receive; they can be
        // database connections, configurations, and other auxiliary arguments. It is similar to
        // `actix_web::Extensions`.
        .dependencies(dptree::deps![bot_interface.clone()])
        // If no handler succeeded to handle an update, this closure will be called.
        .default_handler(|upd| async move {
            log::warn!("Unhandled update: {:?}", upd);
        })
        // If the dispatcher fails for some reason, execute this handler.
        .error_handler(LoggingErrorHandler::with_custom_text(
            "An error has occurred in the dispatcher",
        ))
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
