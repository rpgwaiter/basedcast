extern crate mpd;

use self::mpd::Client;
use dotenv::dotenv;
use std::env;


pub fn mpd_connect() -> Result<mpd::Client, mpd::error::Error> {
    dotenv().expect(".env file not found");
    let mpd_url = env::var("MPD_URL").unwrap();
    let mpd_port = env::var("MPD_PORT").unwrap();
    let mpd_addr = format!("{}:{}", &mpd_url, &mpd_port);
    Client::connect(&mpd_addr)
}