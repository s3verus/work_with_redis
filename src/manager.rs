use crate::config::RedisConfig;
use crate::dao::*;
use lazy_static::lazy_static;
use redis::RedisError;
use std::error::Error;
use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::Mutex;
use std::sync::MutexGuard;

// Declaring a MutStatic
lazy_static! {
    // we don't need Mutex, i'll remove it later
    pub static ref REDIS_LIST: Mutex<Vec<String>> = Mutex::new(vec![]);
}

pub fn set_value(config: RedisConfig) -> Vec<String> {
    let conn = connect(config.clone()).unwrap(); // TODO first edit get_items to solve it
    let list = get_items("block_list", conn).unwrap();
    list
}

pub fn block_domain(
    domain: &str,
    mut conn: redis::Connection,
    config: RedisConfig,
) -> Result<(), RedisError> {
    let _: () = add_items("block_list", domain, &mut conn)?;
    // Resetting a MutStatic
    REDIS_LIST.lock().unwrap().clear();
    REDIS_LIST.lock().unwrap().append(&mut set_value(config));
    Ok(())
}

pub fn is_exists(domain: &String) -> bool {
    // Using a MutStatic
    let result = REDIS_LIST.lock().unwrap();
    if result.contains(domain) {
        true
    } else {
        false
    }
}

pub fn get_second(in_string: &String) -> String {
    // slice string two part and return second part
    let mut splitter = in_string.splitn(2, '.');
    splitter.next().unwrap();
    let second = splitter.next().unwrap();
    second.to_string()
}

pub fn is_exists_rec(domain: &String, result: MutexGuard<Vec<String>>) -> bool {
    // time over and fastest way to implement is recursive
    // but, i think should use trie data structure for better performance
    if result.contains(domain) {
        true
    } else {
        let domain_vec: Vec<&str> = domain.split(".").collect();
        if domain_vec.len() == 2 {
            return false;
        };
        let domain = get_second(domain);
        //println!("{}", domain);
        is_exists_rec(&domain, result)
    }
}

pub fn release_domain(
    domain: &str,
    mut conn: redis::Connection,
    config: RedisConfig,
) -> Result<(), RedisError> {
    let _: () = remove_items("block_list", domain, &mut conn)?;
    // Resetting a MutStatic
    REDIS_LIST.lock().unwrap().clear();
    REDIS_LIST.lock().unwrap().append(&mut set_value(config));
    Ok(())
}

pub fn handle_connection(mut stream: TcpStream, config: RedisConfig) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 256];
    let post_block = b"POST /block HTTP/1.1\r\n";
    let post_check = b"POST /check HTTP/1.1\r\n";
    let post_release = b"POST /release HTTP/1.1\r\n";
    let conn = connect(config.clone())?;

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
        if is_exists(&site) {
            "HTTP/1.1 200 OK\r\nContent-Length: 19\r\n\r\nit's in block list!"
        } else {
            block_domain(&site, conn, config)?;
            "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nsite blocked!"
        }
    } else if buffer.starts_with(post_check) {
        let result = REDIS_LIST.lock().unwrap();
        if is_exists_rec(&site, result) {
            "HTTP/1.1 200 OK\r\nContent-Length: 19\r\n\r\nit's in block list!"
        } else {
            "HTTP/1.1 200 OK\r\nContent-Length: 24\r\n\r\nnot found in block list!"
        }
    } else if buffer.starts_with(post_release) {
        if is_exists(&site) {
            release_domain(&site, conn, config)?;
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_second() {
        let in_string = String::from("a.b.domain.tld");
        assert_eq!(get_second(&in_string), "b.domain.tld".to_string());
    }
}
