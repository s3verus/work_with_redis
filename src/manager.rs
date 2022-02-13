use redis::RedisError;
use std::error::Error;
use std::io::prelude::*;
use std::net::TcpStream;
use crate::dao::*;
use crate::config::DB;

#[derive(Clone)]
pub struct List {
    pub block: Vec<String>,
}

pub fn block_domain(domain: &str, conn: redis::Connection) -> Result<(), RedisError> {
    let _: () = add_items("block_list", domain, conn)?; 
    Ok(())
}

pub fn is_exists(domain: &String, conn: redis::Connection) -> Result<bool, RedisError> {
    let block_list: Vec<String> = get_items("block_list", conn)?;

    if block_list.contains(domain) {
        Ok(true)
    } else {
        Ok(false)
    }
}
pub fn is_exists2(domain: &String, list: &List) -> bool {
    if list.block.contains(domain) {
        true
    } else {
        false
    }
}
pub fn handle_connection(mut stream: TcpStream, config: DB, list: &List ) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 256];
    let post_block = b"POST /block HTTP/1.1\r\n";
    let post_check = b"POST /check HTTP/1.1\r\n";
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
        block_domain(&site, conn)?;
        "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nsite blocked!"
    } else if buffer.starts_with(post_check) {
        // let result = is_exists(&site, conn)?;
        let result = is_exists2(&site, list);
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
