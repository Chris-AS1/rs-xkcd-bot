use anyhow::Context;

use crate::errors::Error;

pub fn connect() -> Result<redis::Connection, Error> {
    let client = redis::Client::open("redis://127.0.0.1/").context("couldn't connect")?;
    let con = client.get_connection().context("connection failed")?;
    Ok(con)
}
