use std::net::TcpListener;
mod config;
use yaml_rust::Yaml;
use std::error::Error;

struct Bind<'a> {
    bind: &'a str,
}

struct DB<'a> {
    host: &'a str,
    port: &'a str,
    pass: &'a str,
    user: &'a str,
}

struct Config<'a> {
    bind: Bind<'a>,
    db: DB<'a>,
}

fn set_config(conf: &Yaml) -> Result<Config, Box<dyn Error>> {
    Ok(Config {
        bind: Bind {
            bind: conf["listener"]["bind"].as_str().unwrap(),
        },
        db: DB {
            host: conf["db"]["host"].as_str().ok_or("127.0.0.1")?,
            port: conf["db"]["port"].as_str().ok_or("6379")?,
            pass: conf["db"]["pass"].as_str().ok_or("")?,
            user: conf["db"]["user"].as_str().ok_or("")?,
        },
    })
}

fn main() {
    let config = config::load_config().unwrap();

    // Multi document support, doc is a yaml::Yaml
    let conf = &config[0];
    
    let config = match set_config(conf) {
        Ok(ok) => ok,
        Err(_) => return (), // TODO handle it
    };

    // read bind from Config.yaml
    let bind = config.bind.bind;
    let listener = TcpListener::bind(bind).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let results = work_with_redis::handle_connection(stream);
        println!("{:?}", results);
    }
}
