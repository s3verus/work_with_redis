use crate::config::RedisConfig;
use redis::Commands;
use redis::RedisError;
use std::error::Error;

pub fn connect(config: RedisConfig) -> Result<redis::Connection, Box<dyn Error>> {
    let conn_url = format!(
        "{}://:{}@{}:{}",
        "redis", config.pass, config.host, config.port
    );
    let result = redis::Client::open(conn_url)?.get_connection()?; // TODO can you remove it?
    Ok(result)
}

pub fn add_items(key: &str, value: &str, conn: &mut redis::Connection) -> Result<(), RedisError> {
    redis::cmd("rpush").arg(key).arg(value).query(&mut *conn)
}

pub fn remove_items(
    key: &str,
    value: &str,
    conn: &mut redis::Connection,
) -> Result<(), RedisError> {
    redis::cmd("lrem")
        .arg(key)
        .arg("0")
        .arg(value)
        .query(&mut *conn)
}

pub fn get_items(list_name: &str, mut conn: redis::Connection) -> Result<Vec<String>, RedisError> {
    conn.lrange(list_name, 0, -1)
}
