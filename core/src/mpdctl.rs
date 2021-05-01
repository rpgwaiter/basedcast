extern crate mpd;

pub use self::mpd::{Client, error::Error};

use crate::settings::load_config;

pub fn mpd_connect() -> Result<Client, Error> {
    let conf = load_config();

    let mpd_url = &conf
        .get("mpd").unwrap()
        .get("url").unwrap()
        .as_str().unwrap();
    let mpd_port = &conf
        .get("mpd").unwrap()
        .get("port").unwrap()
        .as_integer().unwrap();
    let mpd_addr = format!("{}:{}", &mpd_url, &mpd_port);
    Client::connect(&mpd_addr)
}

// TODO: take a bool to run radioscan
pub fn init() -> Result<Client, Error> {

    let mut mpc = mpd_connect()?;

    match mpc.login("password") {
        Ok(_client) => println!("Connected to MPD!"),
        Err(error) => panic!("Unable to connect to mpd: {:?}", error),
    };

    mpc.volume(100)?;
    mpc.play()?;
    Ok(mpc)
}

// TODO: fn play_if_not_playing