extern crate mpd;

pub use self::mpd::{Client, error::Error};
use dotenv::dotenv;

pub fn mpd_connect() -> Result<Client, Error> {
    dotenv().ok();
    let mpd_url = dotenv!("MPD_URL");
    let mpd_port = dotenv!("MPD_PORT");
    let mpd_addr = format!("{}:{}", &mpd_url, &mpd_port);
    Client::connect(&mpd_addr)
}

// TODO: take a bool to run radioscan
pub fn init() -> Result<Client, Error> {

    let mut mpc = mpd_connect()?;

    match mpc.login("password") {
        Ok(_client) => println!("Connected to MPD!"),
        Err(error) => panic!("Unable to connect to mpd: {:?}", error),
    };

    mpc.volume(100)?;
    mpc.play()?;
    Ok(mpc)
}

// TODO: fn play_if_not_playing