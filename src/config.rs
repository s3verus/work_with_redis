use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::Read;

#[derive(Clone, Deserialize, Debug)]
pub struct Listener {
    pub bind: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct DB {
    pub host: String,
    pub port: String,
    pub pass: String,
    pub user: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub listener: Listener,
    pub db: DB,
}

pub fn load_config() -> Result<Config, Box<dyn Error>> {
    // Open file
    let mut file = File::open("Config.json")?;

    // Read the file contents into a string
    let mut s = String::new();
    file.read_to_string(&mut s)?;

    let config: Config = serde_json::from_str(&s)?;

    // println!("config: {:?}", config);
    Ok(config)
}
