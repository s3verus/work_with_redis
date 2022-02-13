use std::net::TcpListener;
use work_with_redis::config::load_config;
use work_with_redis::manager::handle_connection;
use work_with_redis::dao::{connect, get_items};
use work_with_redis::manager::List;

fn main() {
    let config = load_config().unwrap();

    let listener = TcpListener::bind(config.listener.bind).unwrap();

    let conn = connect(config.db.clone()).unwrap();
    let list = List { 
        block: get_items("block_list", conn).unwrap(),
    };

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let results = handle_connection(stream, config.db.clone(), &list);
        println!("{:?}", results);
    }
}
