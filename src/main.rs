use std::net::TcpListener;
use work_with_redis::config::load_config;
use work_with_redis::manager::handle_connection;
use work_with_redis::manager::{MyStruct, MY_STRUCT};

fn main() {
    let config = load_config().unwrap();

    let listener = TcpListener::bind(config.listener.bind).unwrap();

    // Setting a MutStatic
    MY_STRUCT.set(MyStruct::update()).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let results = handle_connection(stream, config.redis.clone()); // TODO can you remove clone?
        println!("{:?}", results);
    }
}
