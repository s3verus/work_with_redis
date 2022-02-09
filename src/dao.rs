use std::error::Error;
#[path = "config.rs"]
mod config;

pub fn connect() -> Result<redis::Connection, Box<dyn Error>> {
    let config = config::load_config()?;
    let conf = &config[0];

    let host = conf["db"]["host"].as_str().ok_or("127.0.0.1")?;
    let port = conf["db"]["port"].as_str().ok_or("6379")?;
    let pass = conf["db"]["pass"].as_str().ok_or("")?;
    let conn_url = format!("{}://:{}@{}:{}", "redis", pass, host, port);

    let result = redis::Client::open(conn_url)?.get_connection()?;
    Ok(result)
}
