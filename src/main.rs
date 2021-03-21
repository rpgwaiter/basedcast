#[macro_use]
extern crate diesel;
extern crate dotenv;

use std::env;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;

mod schema;
mod models;
mod mpdctl;
mod radiofiles;

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Please set DATABASE_URL in your .env");
    let _pg = PgConnection::establish(&database_url).unwrap();

    // let song = models::NewSong {
    //     title:     &institle,
    //     game:      Some(&insgame),
    //     system:    Some(&inssystem),
    //     is_public: &insis_public,
    //     bitrate:   &insbitrate,
    //     duration:  &insduration,
    //     filesize:  &insfilesize,
    //     filename:  &insfilename,
    //     fullpath:  &insfullpath,
    // };

    // if models::Song::insert(song, &pg) {
    //     println!("Added!");
    // } else {
    //     println!("Failed!");
    // };
    
    let mut mpc = mpdctl::mpd_connect().unwrap();
    match mpc.login("password") { // Auth with MPD server
        Ok(_client) => println!("Connected to MPD!"),
        Err(error) => panic!("Unable to connect to mpd: {:?}", error),
    };
    mpc.volume(100).unwrap();
    mpc.play().unwrap();
    radiofiles::get_radiofiles();
}

// Folder Structure: /system/game name (year)/song1.wav