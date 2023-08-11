use anyhow::Context;

use crate::{errors::Error, configuration::Settings};

pub fn connect() -> Result<redis::Connection, Error> {
    let settings = Settings::new().unwrap();
    let client = redis::Client::open(format!("redis://{}", settings.database.url)).context("couldn't connect")?;
    let con = client.get_connection().context("connection failed")?;
    Ok(con)
}
