use crate::{configuration::Settings, errors::Error};
use anyhow::Context;
use redis::{self, Commands, SetExpiry, SetOptions};
use std::{thread, time};

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
            thread::sleep(time::Duration::from_secs(settings.database.retry_timeout));
            log::warn!(
                "connection failed, waiting {} seconds before reconection",
                settings.database.retry_timeout
            );
        } else {
            max_retries = 0;
        }
    }

    Ok(con.unwrap())
}

pub fn consume_daily(
    mut con: redis::Connection,
    settings: Settings,
    username: String,
) -> Result<(), Error> {
    let is_limited: bool = con.exists(username.clone()).unwrap();

    if is_limited {
        log::warn!("{} is rate-limited", username);
        return Err(Error::RateLimitError);
    }

    let opts = SetOptions::default().with_expiration(SetExpiry::EX(
        time::Duration::from_secs(settings.database.rate_limit)
            .as_secs()
            .try_into()
            .unwrap(),
    ));

    let _: () = con
        .set_options(username, true, opts)
        .context("set failed")?;
    Ok(())
}
