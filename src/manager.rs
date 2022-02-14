use crate::config::load_config;
use crate::config::RedisConfig;
use crate::dao::*;
use lazy_static::lazy_static;
use mut_static::MutStatic;
use redis::RedisError;
use std::error::Error;
use std::io::prelude::*;
use std::net::TcpStream;

#[derive(Debug, Clone)]
pub struct MyStruct {
    pub list: Vec<String>,
}

impl MyStruct {
    pub fn update() -> Self {
        let config = load_config().unwrap(); // TODO can you remove it?
        let conn = connect(config.redis.clone()).unwrap(); // TODO can you remove it?
        let list = get_items("block_list", conn).unwrap();
        Self { list }
    }
}

// Declaring a MutStatic
lazy_static! {
    pub static ref MY_STRUCT: MutStatic<MyStruct> = MutStatic::new();
}

pub fn block_domain(domain: &str, mut conn: redis::Connection) -> Result<(), RedisError> {
    let _: () = add_items("block_list", domain, &mut conn)?;
    // Resetting a MutStatic
    let mut my_struct = MY_STRUCT.write().unwrap();
    *my_struct = MyStruct::update();
    Ok(())
}

pub fn is_exists(domain: &String) -> bool {
    // Using a MutStatic
    let result = MY_STRUCT.read().unwrap();
    let result = result.list.contains(domain);
    if result {
        true
    } else {
        false
    }
}

pub fn release_domain(domain: &str, mut conn: redis::Connection) -> Result<(), RedisError> {
    let _: () = remove_items("block_list", domain, &mut conn)?;
    // Resetting a MutStatic
    let mut my_struct = MY_STRUCT.write().unwrap();
    *my_struct = MyStruct::update();
    Ok(())
}

pub fn handle_connection(mut stream: TcpStream, config: RedisConfig) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 256];
    let post_block = b"POST /block HTTP/1.1\r\n";
    let post_check = b"POST /check HTTP/1.1\r\n";
    let post_release = b"POST /release HTTP/1.1\r\n";
    let conn = connect(config)?;

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
        let result = is_exists(&site);
        if result {
            "HTTP/1.1 200 OK\r\nContent-Length: 19\r\n\r\nit's in block list!"
        } else {
            block_domain(&site, conn)?;
            "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nsite blocked!"
        }
    } else if buffer.starts_with(post_check) {
        let result = is_exists(&site);
        if result {
            "HTTP/1.1 200 OK\r\nContent-Length: 19\r\n\r\nit's in block list!"
        } else {
            "HTTP/1.1 200 OK\r\nContent-Length: 24\r\n\r\nnot found in block list!"
        }
    } else if buffer.starts_with(post_release) {
        let result = is_exists(&site);
        if result {
            release_domain(&site, conn)?;
            "HTTP/1.1 200 OK\r\nContent-Length: 14\r\n\r\nsite released!"
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
