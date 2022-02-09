use std::fs::File;
use std::io::Read;
extern crate yaml_rust;
use std::error::Error;
use yaml_rust::Yaml;
use yaml_rust::YamlLoader;

pub fn load_config() -> Result<Yaml, Box<dyn Error>> {
    // Open file
    let mut file = File::open("Config.yaml")?;

    // Read the file contents into a string
    let mut s = String::new();
    file.read_to_string(&mut s)?;

    let docs = YamlLoader::load_from_str(&s)?;

    // Multi document support, doc is a yaml::Yaml
    Ok(yaml_rust::Yaml::Array(docs))
}
