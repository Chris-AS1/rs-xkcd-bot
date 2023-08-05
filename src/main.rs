use dotenv::dotenv;
use oxotly_bot::utils::spawn;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    log::info!("Starting Oxotly Bot...");

    spawn().await;
}
