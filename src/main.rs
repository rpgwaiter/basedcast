extern crate mpd;
#[macro_use]
extern crate diesel;
extern crate dotenv;

use mpd::Client;
use std::env;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;

mod schema;
mod models;

fn mpd_connect() -> Result<mpd::Client, mpd::error::Error> {
    dotenv().expect(".env file not found");
    let mpd_url = env::var("MPD_URL").unwrap();
    let mpd_port = env::var("MPD_PORT").unwrap();
    let mpd_addr = format!("{}:{}", &mpd_url, &mpd_port);
    Client::connect(&mpd_addr)
}

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Please set DATABASE_URL in your .env");
    let pg = PgConnection::establish(&database_url).unwrap();

    let medium = models::NewMedia {
        title: String::from("test"),
        system: String::from("Playstation"),
        is_public: true,
        bitrate: 123456,
        duration: 4,
        filesize: 1233456,
        filename: String::from("1.mp3"),
        fullpath: String::from("/mnt/public/Radio/khrip/playstation/1xtreme/1.mp3"),
    };

    if models::Media::insert(medium, &pg) {
        println!("Added!");
    } else {
        println!("Failed!");
    };
    
    let mut mpc = mpd_connect().unwrap();
    match mpc.login("password") { // Auth with MPD server
        Ok(_client) => println!("Connected to MPD!"),
        Err(error) => panic!("Unable to connect to mpd: {:?}", error),
    };
    mpc.volume(100).unwrap();
    mpc.play().unwrap();
}

// Folder Structure: /system/game name (year)/song1.wav