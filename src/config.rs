use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::Read;

#[derive(Clone, Deserialize, Debug)]
pub struct ListenerConfig {
    pub bind: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct RedisConfig {
    pub host: String,
    pub port: String,
    pub pass: String,
    pub user: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub listener: ListenerConfig,
    pub redis: RedisConfig,
}

pub fn load_config(path: &String) -> Result<Config, Box<dyn Error>> {
    // Open file
    let mut file = File::open(path)?;

    // Read the file contents into a string
    let mut s = String::new();
    file.read_to_string(&mut s)?;

    let config: Config = serde_json::from_str(&s)?;

    Ok(config)
}
