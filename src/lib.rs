use redis::Commands;
use std::io::prelude::*;
use std::net::TcpStream;

pub fn connect() -> redis::Connection {
    let redis_host_name = "127.0.0.1:6379";
    let redis_password = "";
    let redis_conn_url = format!("{}://:{}@{}", "redis", redis_password, redis_host_name);

    redis::Client::open(redis_conn_url)
        .expect("Invalid connection URL")
        .get_connection()
        .expect("failed to connect to Redis")
}

pub fn block_domain(domain: &str) {
    let mut conn = connect();
    let _: () = redis::cmd("rpush")
        .arg("block_list")
        .arg(domain)
        .query(&mut conn)
        .expect("failed to rpush domain!");

    //println!("domain {}, blocked!", domain);
}

pub fn release_domain(domain: &str) {
    println!("domain {}, released!", domain);
}

pub fn read_block_list() -> Vec<String> {
    let mut conn = connect();
    let block_list: Vec<String> = conn
        .lrange("block_list", 0, -1)
        .expect("failed to execute LRANGE for 'items'");

    block_list
}

pub fn is_exists(domain: &String) -> bool {
    let block_list = read_block_list();
    if block_list.contains(domain) {
        true
    } else {
        false
    }
}

pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 256];
    let post_block = b"POST /block HTTP/1.1\r\n";
    let post_check = b"POST /check HTTP/1.1\r\n";

    stream.read(&mut buffer).unwrap();
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
        block_domain(&site);
        "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nsite blocked!"
    } else if buffer.starts_with(post_check) {
        if is_exists(&site) {
            "HTTP/1.1 200 OK\r\nContent-Length: 19\r\n\r\nit's in block list!"
        } else {
            "HTTP/1.1 200 OK\r\nContent-Length: 24\r\n\r\nnot found in block list!"
        }
    } else {
        "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 14\r\n\r\nwrong request!"
    };

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
