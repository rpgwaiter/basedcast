extern crate mpd;

use mpd::Client;
use std::net::TcpStream;
use std::env;

fn main() {
    let mut conn = Client::connect(format!("{}:{}", env::var(MPD_URL), env::var(MPD_PORT) )).unwrap();
    conn.volume(100).unwrap();
    conn.load("My Lounge Playlist", ..).unwrap();
    conn.play().unwrap();
    println!("Status: {:?}", conn.status());
}
