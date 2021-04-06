#[macro_use]
extern crate diesel;
extern crate dotenv;

use std::env;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use models::{NewSong, Song};


mod schema;
mod models;
mod mpdctl;
mod radiofiles;

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Please set DATABASE_URL in your .env");
    let pg = PgConnection::establish(&database_url).unwrap();

    let mut mpc = mpdctl::mpd_connect().unwrap();
    match mpc.login("password") { // Auth with MPD server
        Ok(_client) => println!("Connected to MPD!"),
        Err(error) => panic!("Unable to connect to mpd: {:?}", error),
    };
    mpc.volume(100).unwrap();
    mpc.play().unwrap(); 

    let radiofiles = radiofiles::get_radiofiles(
        &env::var("RADIOFILES_ROOT").expect("Set RADIOFILES_ROOT")
    );

    radiofiles::upsert_db(radiofiles, &pg).unwrap(); // scan files

    // for song in Song::all(&pg) {
    //     let title = song.title.to_string();
    //     let path: &str = &song.fullpath.to_string();
    //     &mpc.pl_push(&title, &path);
    //     println!("pushed {:?}", &song.title);
    // };

    println!("{} Songs in the database. Starting scan...", &mpc.stats().unwrap().songs);
    &mpc.rescan().unwrap();
}

// Folder Structure: /system/game name (year)/song1.wav