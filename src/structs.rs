use yaml_rust::Yaml;
use std::error::Error;

pub struct Bind<'a> {
    pub bind: &'a str,
}

pub struct DB<'a> {
    pub host: &'a str,
    pub port: &'a str,
    pub pass: &'a str,
    pub user: &'a str,
}

pub struct Config<'a> {
    pub bind: Bind<'a>,
    pub db: DB<'a>,
}

// should change to impl Config
pub fn set_config(conf: &Yaml) -> Result<Config, Box<dyn Error>> {
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
