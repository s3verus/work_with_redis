use std::env;
use std::net::TcpListener;
use work_with_redis::config::load_config;
use work_with_redis::manager::{handle_connection, set_value};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = if args.len() == 2 {
        load_config(&args[1]).unwrap()
    } else {
        load_config(&"Config.json".to_string()).unwrap()
    };

    let listener = TcpListener::bind(config.listener.bind).unwrap();

    // Setting a MutStatic
    set_value(config.redis.clone());

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let results = handle_connection(stream, config.redis.clone());
        println!("{:?}", results);
    }
}
