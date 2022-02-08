use std::net::TcpListener;

fn main() {
    
    let docs = work_with_redis::yaml_object().unwrap();

    // Multi document support, doc is a yaml::Yaml
    let doc = &docs[0];

    // read bind from Config.yaml
    let bind = doc["config"]["bind"].as_str().unwrap();
    let listener = TcpListener::bind(bind).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let results = work_with_redis::handle_connection(stream);
        println!("{:?}", results);
    }
}
