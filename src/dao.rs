use std::error::Error;
use crate::config::DB;

pub fn connect(config: DB) -> Result<redis::Connection, Box<dyn Error>> {
    let host = config.host;
    let port = config.port;
    let pass = config.pass;
    let conn_url = format!("{}://:{}@{}:{}", "redis", pass, host, port);

    let result = redis::Client::open(conn_url)?.get_connection()?;
    Ok(result)
}
