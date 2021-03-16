extern crate mpd;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
//extern crate dotenv;

use mpd::Client;
use std::net::TcpStream;
use std::env;

fn main() {
    let mpd_url = match env::var("MPD_URL") {
        Ok(val) => val,
        Err(e) => panic!("could not find {}: {}", MPD_URL, e),
    };

    let mut conn = Client::connect(println!("{}:{}", 
        //env::var(MPD_URL), 
        "127.0.0.1",
        //env::var(MPD_PORT) 
        "6600"
    )).unwrap();
    conn.volume(100).unwrap();
    conn.load("My Lounge Playlist", ..).unwrap();
    conn.play().unwrap();
    println!("Status: {:?}", conn.status());
}
