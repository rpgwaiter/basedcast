extern crate mpd;
//#[macro_use]
extern crate diesel;

// use mpd::Client;
//use std::net::TcpStream;
use std::env;
use envmnt::{ExpandOptions, ExpansionType};
// use std::net::{SocketAddr, ToSocketAddrs};

//static MPD_ADDR: &'static str = println!("{}:{}", env::var("MPD_URL"), env::var("MPD_PORT"));

fn main() {

    // let mut conn = Client::connect(&MPD_ADDR).unwrap();
    // conn.volume(100).unwrap();
    // conn.load("My Lounge Playlist", ..).unwrap();
    // conn.play().unwrap();
    // println!("Status: {:?}", conn.status());
    println!("{:?}", envmnt::get_or("MPD_URL", "DEFAULT_VALUE"))
}
