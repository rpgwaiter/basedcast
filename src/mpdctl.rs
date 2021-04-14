extern crate mpd;

use self::mpd::{Client, error::Error};
use dotenv::dotenv;
use std::env;

pub fn mpd_connect() -> Result<Client, Error> {
    dotenv().expect(".env file not found");
    let mpd_url = env::var("MPD_URL").expect("Set MPD_URL!");
    let mpd_port = env::var("MPD_PORT").expect("Set MPD_PORT!");
    let mpd_addr = format!("{}:{}", &mpd_url, &mpd_port);
    Client::connect(&mpd_addr)
}

pub fn init() -> Result<Client, Error> {
    use crate::radiofiles::get_radiofiles;

    let mut mpc = mpd_connect()?;

    match mpc.login("password") { // Auth with MPD server
        Ok(_client) => println!("Connected to MPD!"),
        Err(error) => panic!("Unable to connect to mpd: {:?}", error),
    };
    mpc.volume(100)?;

    let radiofiles = get_radiofiles(
        &env::var("RADIOFILES_ROOT").expect("Set RADIOFILES_ROOT")
    );

    crate::radiofiles::upsert_db(&radiofiles).unwrap(); // scan files into db

    // Gather list of all songs mpd knows about
    let mut query = mpd::Query::new();
    let size = *&radiofiles.len() as u32;
    let window: mpd::search::Window = (0u32, (size)).into();
    let finished = query.and(mpd::Term::LastMod, "0");

    // adds all songs to the queue
    &mpc.find(finished, window).unwrap().iter().for_each(|x| {&mpc.push(x);});
    // save queue as 'radio' playlist
    &mpc.save("radio");

    mpc.play()?;
    
    Ok(mpc)
}