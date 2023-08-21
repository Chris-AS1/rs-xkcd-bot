use oxotly_bot::configuration::Settings;
use oxotly_bot::database::{connect, consume_daily};
use oxotly_bot::utils::{build_settings, setup};
use redis::Commands;
use uuid::Uuid;

#[test]
fn redis_connection() {
    setup();
    connect().unwrap();
}

#[test]
fn redis_set_read_delete_value() {
    let mut con = connect().unwrap();

    let _: () = con.set("foo", 42).unwrap();
    let result: i32 = con.get("foo").unwrap();
    let _: () = con.del("foo").unwrap();

    assert_eq!(42, result);
}

#[test]
fn rate_limit_user() {
    setup();
    let con = connect().unwrap();
    let settings: Settings = build_settings().unwrap();
    let username = format!("{}", Uuid::new_v4()).to_string();

    let res = consume_daily(con, settings, username);
    assert!(res.is_ok())
}
