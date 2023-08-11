use oxotly_bot::database::connect;
use redis::Commands;

#[test]
fn redis_connection_builder() {
    connect().expect("conection failed");
}

#[test]
fn redis_connection() {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    client.get_connection().unwrap();
}

#[test]
fn redis_set_and_read_value() {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();

    let _: () = con.set("foo", 42).unwrap();

    let result: i32 = con.get("foo").unwrap();

    println!("{:?}", result);
}
