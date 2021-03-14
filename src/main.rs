extern crate mpd;

use mpd::Client;
use std::net::TcpStream;
use std::env;

fn main {
    let mut conn = Client::connect("127.0.0.1:6600").unwrap(); // use env vars eventually print_env_var("");
    conn.volume(100).unwrap();
    conn.load("My Lounge Playlist", ..).unwrap();
    conn.play().unwrap();
    println!("Status: {:?}", conn.status());
}
