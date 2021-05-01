use std::{fs::File, io::prelude::* };
use std::path::Path;
use toml::Value;

pub fn load_config() -> Value {

    let mut input = String::new();
    let path = Path::new("settings.toml");

    File::open(path)
        .and_then(|mut f| f.read_to_string(&mut input))
        .expect("error parsing settings.toml");

    input.parse::<Value>().unwrap()
}