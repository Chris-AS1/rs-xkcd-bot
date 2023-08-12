use oxotly_bot::utils::{spawn, setup};

#[tokio::main]
async fn main() {
    setup();
    log::info!("Starting Oxotly Bot...");

    spawn().await;
}
