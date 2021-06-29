// This is used to update a running basedcast library
// Eventually I'd like basedcast to scan for changes
extern crate id3;
extern crate mp3_duration;
extern crate regex;
extern crate globwalk;
extern crate indicatif;
extern crate checksums;


use basedcast_core::mpdctl::mpd_connect;
use basedcast_api::db::models::song::{Song, NewSong};
use std::path::PathBuf;
use basedcast_api::db::connect;
use basedcast_api::db::PgPool;
use basedcast_core::settings::load_config;
use self::id3::Tag;

pub fn get_radiofiles(root: &str) -> Vec<PathBuf> {
    globwalk::glob(&format!("{}/**/*.mp3", &root))
        .unwrap()
        .map(|x| x.unwrap().path().to_path_buf())
        .collect()
}

fn parse_path(file: &PathBuf) -> (String, String, i32) {
    use self::regex::Regex;

    let conf = load_config();

    let radiofiles_dir = &conf
        .get("mpd").unwrap()
        .get("radiofiles_root").unwrap()
        .as_str().unwrap();

    let postprefix = &file.to_str().unwrap().trim_start_matches(&format!("{}/", radiofiles_dir));

    let mut splits = postprefix.split(|c| c == '/');
    let system = splits.next().unwrap();

    let ginput = splits.next().unwrap();
    let gex = Regex::new(r"(?P<game>.*)\((?P<year>\d{4})\)").unwrap(); //extract game and year from folder
    let rxout = gex.captures(&ginput).unwrap();
    
    (system.to_string(),
    rxout["game"].trim().to_string(), 
    rxout["year"].parse().unwrap())
}

fn fill_song_info(s: &PathBuf) -> NewSong {
    use std::str::FromStr;
    let mut song = NewSong::default();
    
    let parsed = parse_path(&s); // grabs (system, game, year)
    song.game = parsed.1;
    song.year = parsed.2;
    song.system = parsed.0;
    song.is_public = true;
    song.duration = mp3_duration::from_path(&s).unwrap().as_millis() as i32;
    song.filesize = std::fs::metadata(&s).unwrap().len() as i32;
    song.bitrate = song.filesize - song.duration;
    song.filename = s.file_name().unwrap().to_os_string().into_string().unwrap();
    song.fullpath = s.to_str().unwrap().to_string();
    song.hash = checksums::hash_file(&s, checksums::Algorithm::from_str("SHA3256").unwrap());
    song.title = s.file_stem().unwrap().to_os_string().into_string().unwrap();
    
    let tags = Tag::read_from_path(&s);
    match tags {
        Ok(t) => {
            if let Some(track) = t.track() { song.track = Some(track as i32) }
            if let Some(artist) = t.artist() { song.artist = Some(artist.to_string()) }
            if let Some(title) = t.title() { song.title = title.to_string() }
        },
        _ => (), // Do nothing if song is missing id3 tags
    };
    song
}

pub fn upsert_db(songs: &Vec<std::path::PathBuf>, pgpool: &PgPool) -> Option<String> {
    use indicatif::{ ProgressBar, ProgressStyle };

    let sty = ProgressStyle::default_bar()
        .template("{bar:40.green/yellow} {pos:>4}/{len:4} {msg}")
        .progress_chars("=> ");

    let pb = ProgressBar::new(songs.len() as u64);
    pb.set_style(sty);
    pb.tick();

    songs.into_iter().for_each(|songpath| {
        let songinfo = fill_song_info(songpath);
        Song::upsert(songinfo.clone(), &pgpool.get().unwrap()).ok();
        pb.set_message(&format!("Scanned {:?}", songinfo.filename));
        pb.inc(1);
    });
    Some(format!("updated songs"))
}

fn main() {
    let mut mpc = mpd_connect().unwrap();
    let conf = load_config();
    let root = &conf
        .get("mpd").unwrap()
        .get("radiofiles_root").unwrap()
        .as_str().unwrap();
    let radiofiles = get_radiofiles(root);

    println!("{:#?}",Tag::read_from_path(&radiofiles[0]).unwrap().artist());

    match mpc.login("password") { // Auth with MPD server
        Ok(_client) => println!("Connected to MPD!"),
        Err(error) => panic!("Unable to connect to mpd: {:?}", error),
    };
    mpc.volume(100).unwrap();
    
    let conn = connect();

    upsert_db(&radiofiles, &conn).unwrap(); // scan files into db

    // Gather list of all songs mpd knows about
    let mut query = mpd::Query::new();
    let size = *&radiofiles.len() as u32;
    let window: mpd::search::Window = (0u32, (size)).into();
    let finished = query.and(mpd::Term::LastMod, "0");

    // adds all songs to the queue
    &mpc.find(finished, window).unwrap().iter().for_each(|x| {&mpc.push(x);});
    
    // save queue as 'radio' playlist, del the old one
    if mpc.playlist("radio").is_err() {
        &mpc.save("radio");
        println!("{:?} songs scanned!", &radiofiles.len());
    } else {
        &mpc.pl_remove("radio");
        &mpc.save("radio");
        println!("{:?} songs scanned!", &radiofiles.len());
    }
}