use std::error::Error;
use crate::config::DB;
use redis::RedisError;
use redis::Commands;

pub fn connect(config: DB) -> Result<redis::Connection, Box<dyn Error>> {
    let host = config.host;
    let port = config.port;
    let pass = config.pass;
    let conn_url = format!("{}://:{}@{}:{}", "redis", pass, host, port);

    let result = redis::Client::open(conn_url)?.get_connection()?;
    Ok(result)
}

pub fn add_items(key: &str, value: &str, conn: &mut redis::Connection) -> Result<(), RedisError> {
    let _: () = redis::cmd("rpush")
        .arg(key)
        .arg(value)
        .query(&mut *conn)?;
    Ok(())
}

pub fn get_items(list_name: &str, mut conn: redis::Connection) -> Result<Vec<String>, RedisError> {
    conn.lrange(list_name, 0, -1)
}
