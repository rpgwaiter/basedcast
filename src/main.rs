extern crate mpd;
//#[macro_use]
extern crate diesel;

use mpd::Client;
//use std::net::TcpStream;
//use envmnt::{ExpandOptions, ExpansionType};
// use std::net::{SocketAddr, ToSocketAddrs};

fn mpd_connect() -> Result<mpd::Client, mpd::error::Error> {
    let mpd_url: &str = &envmnt::get_or("MPD_URL", "127.0.0.1");
    let mpd_port: &str = &envmnt::get_or("MPD_PORT", "6600");
    let mpd_addr: &str = &format!("{}:{}", &mpd_url, &mpd_port);
    Client::connect(&mpd_addr)
}

fn main() {
    let mut conn = mpd_connect().unwrap();
    match conn.login("password") {
        Ok(_client) => println!("Connected to MPD! Status: {:?}", conn.status()),
        Err(error) => panic!("Unable to connect to mpd: {:?}", error),
    };
    conn.volume(100).unwrap();
    //conn.load("My Lounge Playlist", ..).unwrap();
    conn.play().unwrap();
    println!("Status: {:?}", conn.status());
}

// Folder Structure: /system/game name (year)/song1.wav