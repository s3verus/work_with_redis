use redis::Commands;
use std::io::prelude::*;
use std::net::TcpStream;
use redis::RedisError;
use std::error::Error;
use std::fs::File;
extern crate yaml_rust;
use yaml_rust::YamlLoader;
use yaml_rust::Yaml;
 
pub fn connect() -> Result<redis::Connection, RedisError> {
    let redis_host_name = "127.0.0.1:6379";
    let redis_password = "";
    let redis_conn_url = format!("{}://:{}@{}", "redis", redis_password, redis_host_name);

    let result = redis::Client::open(redis_conn_url)?
        .get_connection()?;
    Ok(result)
}

pub fn block_domain(domain: &str, mut conn: redis::Connection) -> Result<(), RedisError> {
    let _: () = redis::cmd("rpush")
        .arg("block_list")
        .arg(domain)
        .query(&mut conn)?;
    Ok(())
}

pub fn is_exists(domain: &String, mut conn: redis::Connection) -> Result<bool, RedisError> {
    let block_list: Vec<String> = conn
        .lrange("block_list", 0, -1)?;
    
    if block_list.contains(domain) {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 256];
    let post_block = b"POST /block HTTP/1.1\r\n";
    let post_check = b"POST /check HTTP/1.1\r\n";
    let conn = connect()?;

    stream.read(&mut buffer)?; 
    println!("\nRequest:\n{}", String::from_utf8_lossy(&buffer[..]));

    // parse data
    let data = String::from_utf8_lossy(&buffer[..]);
    let data: Vec<&str> = data.split("\r\n").collect();
    let site = match data.last() {
        Some(x) => x.replace("\u{0}", ""),
        None => "request with empty or invalid data!".to_string(),
    };

    // Validating the Request and Selectively Responding
    let response = if buffer.starts_with(post_block) {
        block_domain(&site, conn)?;
        "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nsite blocked!"
    } else if buffer.starts_with(post_check) {
        let result = is_exists(&site, conn)?;
        if result {
            "HTTP/1.1 200 OK\r\nContent-Length: 19\r\n\r\nit's in block list!"
        } else {
            "HTTP/1.1 200 OK\r\nContent-Length: 24\r\n\r\nnot found in block list!"
        }
    } else {
        "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 14\r\n\r\nwrong request!"
    };

    stream.write(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}

pub fn yaml_object() -> Result<Yaml, Box<dyn Error>> {
    // Open file
    let mut file = File::open("Config.yaml")?;

    // Read the file contents into a string
    let mut s = String::new();
    file.read_to_string(&mut s)?;

    let docs = YamlLoader::load_from_str(&s)?;

    // Multi document support, doc is a yaml::Yaml
    Ok(yaml_rust::Yaml::Array(docs))
}
