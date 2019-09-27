use std::time;
use redis;
use redis::{Commands};

pub fn store_health_check(conn: &mut redis::Connection, server_name: &str, healthy: bool,  expire_in: time::Duration) -> redis::RedisResult<bool> {
    conn.set_ex::<&str, bool, bool>(server_name, healthy, expire_in.as_secs() as usize)
}

#[test]
fn test_set() {
    let client = redis::Client::open("redis://127.0.0.1").expect("failed to open redis connection");
    let mut conn = client.get_connection().expect("failed to get connection");

    assert_eq!(store_health_check(&mut conn, "recurse.com", true, time::Duration::new(20, 0)).unwrap(), true);
}
