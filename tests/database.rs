use oxotly_bot::database::connect;
use redis::Commands;

#[test]
fn redis_connection() {
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
