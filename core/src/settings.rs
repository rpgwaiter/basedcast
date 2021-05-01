use serde_derive::Deserialize;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

#[derive(Deserialize)]
struct Api {
    url: String,
}

#[derive(Deserialize)]
struct Db {
    url: String,
}

#[derive(Deserialize)]
struct Mpd {
    url: String,
    port: i32,
    radiofiles_root: String,
}

pub fn load_config() {

    let mut input = String::new();
    let path = Path::new("settings.toml");

    println!("path: {:#?}", &path);

    let config_file = File::open(path)
        .and_then(|mut f| f.read_to_string(&mut input))
        .expect("error parsing settings.toml");

    println!("config_file: {:#?}", input);
    
    //toml::toml!(config_file)
}