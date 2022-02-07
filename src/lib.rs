use redis::Commands;
use std::io::prelude::*;
use std::net::TcpStream;
use redis::RedisError;

pub fn connect() -> redis::Connection {
    let redis_host_name = "127.0.0.1:6379";
    let redis_password = "";
    let redis_conn_url = format!("{}://:{}@{}", "redis", redis_password, redis_host_name);

    redis::Client::open(redis_conn_url)//?
        .expect("Invalid connection URL")
        .get_connection()//?
        .expect("failed to connect to Redis")
}

pub fn block_domain(domain: &str) -> Result<(), RedisError> {
    let mut conn = connect();
    let _: () = redis::cmd("rpush")
        .arg("block_list")
        .arg(domain)
        .query(&mut conn)?;
    Ok(())
}

pub fn is_exists(domain: &String) -> Result<bool, RedisError> {
    let mut conn = connect();
    let block_list: Vec<String> = conn
        .lrange("block_list", 0, -1)?;
    
    if block_list.contains(domain) {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn handle_connection(mut stream: TcpStream) -> Result<(), RedisError> {
    let mut buffer = [0; 256];
    let post_block = b"POST /block HTTP/1.1\r\n";
    let post_check = b"POST /check HTTP/1.1\r\n";

    stream.read(&mut buffer)?; // TODO handle it, has different error type!
    println!("Request:\n{}", String::from_utf8_lossy(&buffer[..]));

    // parse data
    let data = String::from_utf8_lossy(&buffer[..]);
    let data: Vec<&str> = data.split("\r\n").collect();
    let site = match data.last() {
        Some(x) => x.replace("\u{0}", ""),
        None => "request with empty or invalid data!".to_string(),
    };

    // Validating the Request and Selectively Responding
    let response = if buffer.starts_with(post_block) {
        match block_domain(&site) {
            Ok(()) => (),
            Err(e) => return Err(e),
        }
        "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nsite blocked!"
    } else if buffer.starts_with(post_check) {
        let result = match is_exists(&site) {
            Ok(boolean) => boolean,
            Err(e) => return Err(e),
        };
        if result {
            "HTTP/1.1 200 OK\r\nContent-Length: 19\r\n\r\nit's in block list!"
        } else {
            "HTTP/1.1 200 OK\r\nContent-Length: 24\r\n\r\nnot found in block list!"
        }
    } else {
        "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 14\r\n\r\nwrong request!"
    };

    stream.write(response.as_bytes())?; // TODO handle
    stream.flush()?; // TODO handle
    Ok(())
}
