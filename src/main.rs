use std::net::TcpListener;

fn main() {
    let config = work_with_redis::load_config().unwrap();

    // Multi document support, doc is a yaml::Yaml
    let conf = &config[0];

    // read bind from Config.yaml
    let bind = conf["listener"]["bind"].as_str().unwrap();
    let listener = TcpListener::bind(bind).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let results = work_with_redis::handle_connection(stream);
        println!("{:?}", results);
    }
}
