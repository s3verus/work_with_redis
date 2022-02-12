use std::error::Error;

pub fn connect(config: Config) -> Result<redis::Connection, Box<dyn Error>> {
    let host = config.db.host;
    let port = config.db.port;
    let pass = config.db.pass;
    let conn_url = format!("{}://:{}@{}:{}", "redis", pass, host, port);

    let result = redis::Client::open(conn_url)?.get_connection()?;
    Ok(result)
}
