use std::{thread, time};

use anyhow::Context;
use redis;

use crate::{configuration::Settings, errors::Error};

pub fn connect() -> Result<redis::Connection, Error> {
    let settings = Settings::new().unwrap();
    let client =
        redis::Client::open(settings.database.get_url()).context("couldn't build client")?;
    let mut con = client.get_connection();

    let mut max_retries = 3;
    while max_retries > 0 {
        if let Err(_) = con {
            max_retries -= 1;
            con = client.get_connection();
            thread::sleep(time::Duration::from_secs(5));
            // log::warn!("connection failed, waiting 5 seconds before reconection");
        } else {
            max_retries = 0;
        }
    }

    Ok(con.unwrap())
}
