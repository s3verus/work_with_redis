use std::net::TcpListener;
use yaml_rust::Yaml;
use std::error::Error;

fn main() {
    let config = config::load_config().unwrap();

    // Multi document support, doc is a yaml::Yaml
    let conf = &config[0];
    
    let config = match structs::set_config(conf) {
        Ok(ok) => ok,
        Err(_) => return (), // TODO handle it
    };

    // read bind from Config.yaml
    let bind = config.bind.bind;
    let listener = TcpListener::bind(bind).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let results = manager::handle_connection(stream, config);
        println!("{:?}", results);
    }
}
