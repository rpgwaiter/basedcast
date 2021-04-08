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

    //radiofiles::upsert_db(radiofiles, &pg).unwrap(); // scan files

    // for song in Song::all(&pg) {
    //     let title = song.title.to_string();
    //     let path: &str = &song.fullpath.to_string();
    //     &mpc.pl_push(&title, &path);
    //     println!("pushed {:?}", &song.title);
    // };

    // fn collect(arguments: I) -> Vec<String> {
    //     let mut output = Vec::<String>::new();
    //     arguments.to_arguments::<_, ()>(&mut |arg| Ok(output.push(arg.to_string()))).unwrap();
    //     output
    // }
    let mut query = mpd::Query::new();
    let mut ugh = Vec::<String>::new();
    let window: mpd::search::Window = (0u32, (radiofiles.len() as u32)).into();
    let finished = query.and(mpd::Term::LastMod, "0");
    //let gone = finished.to_arguments::<_, ()>(&mut |arg| Ok(ugh.push(arg.to_string()))).unwrap();


    //println!("{} Songs in the database. Starting scan...", &output);
    //&mpc.rescan().unwrap();

    //&mpc.play(); 
    // adds all songs to the queue
    &mpc.find(finished, window).unwrap().iter().for_each(|x| {&mpc.push(x);});
    // queue -> 'radio' playlist
    &mpc.save("radio");
    &mpc.play();
    println!("Status: {:#?}", &mpc.status());
}

// Folder Structure: /system/game name (year)/song1.wav