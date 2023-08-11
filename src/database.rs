use anyhow::Context;

use crate::{errors::Error, configuration::Settings};

pub fn connect() -> Result<redis::Connection, Error> {
    let settings = Settings::new().unwrap();
    let client = redis::Client::open(settings.database.get_url()).context("couldn't connect")?;
    let con = client.get_connection().context("connection failed")?;
    Ok(con)
}
