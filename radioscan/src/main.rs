// This is used to update a running basedcast library
// Eventually I'd like basedcast to scan for changes

#[macro_use]
extern crate dotenv_codegen;
extern crate rayon;
extern crate metadata;
extern crate regex;
extern crate globwalk;
extern crate indicatif;

use basedcast_core::mpdctl::mpd_connect;
use basedcast_api::db::models::song::{Song, NewSong};
use std::path::PathBuf;
use std::{io, convert::TryFrom};
use self::metadata::MediaFileMetadata;
use basedcast_api::db::connect;
use basedcast_api::db::PgPool;


pub fn get_radiofiles(root: &str) -> Vec<PathBuf> {
    globwalk::glob(&format!("{}/**/*.mp3", &root))
        .unwrap()
        .map(|x| x.unwrap().path().to_path_buf())
        .collect()
}

pub fn get_mediainfo(file: &PathBuf) -> Result<MediaFileMetadata, io::Error> {
    let build_media_file_metadata = |file: &PathBuf| -> io::Result<MediaFileMetadata> {
        let mut meta = MediaFileMetadata::new(&file)?;
        meta.include_checksum(true)?
            .include_tags(true);
        Ok(meta)
    };
    build_media_file_metadata(&file)
}

fn parse_tags(tags: Vec<(String, String)>) -> (String, String, i32) {
    let mut artist = String::new(); let mut title = String::new(); let mut track = 0;
    for tag in tags {
        match tag.0.as_str() {
            "artist" => artist.push_str(&tag.1),
            "title"  => title.push_str(&tag.1),
            "track"  => track = tag.1.parse::<i32>().unwrap(),
            _ => (),
        }
    } (artist, title, track)
}

fn parse_path(file: &PathBuf) -> (String, String, i32) {
    use self::regex::Regex;

    let root = dotenv!("RADIOFILES_ROOT");
    let postprefix = &file.to_str().unwrap().trim_start_matches(&format!("{}/", root));

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
    let mut song = NewSong::default();

    let mediainfo = get_mediainfo(&s).unwrap();
    let tags = parse_tags(get_mediainfo(&s).unwrap().tags);
    let parsed = parse_path(&s); // grabs (system, game, year)

    song.title = Option::as_ref(&mediainfo.title).unwrap().to_string();
    song.track = Some(tags.2);
    song.game = parsed.1; 
    song.artist = Some(tags.0);
    song.year = parsed.2;
    song.system = parsed.0;
    song.is_public = true;
    song.bitrate =  if let Some(b) = mediainfo._bit_rate { b as i32 } else { 0 };
    song.duration = if let Some(d) = mediainfo._duration { d as i32 } else { 0 };
    song.filesize = i32::try_from(mediainfo.file_size).unwrap();
    song.filename = mediainfo.file_name;
    song.fullpath = mediainfo.path;
    //song.hash = uuid::Uuid::parse_str(truncate(Option::as_ref(&mediainfo.hash).unwrap(), 32)).unwrap() /*as diesel::pg::types::sql_types::Uuid*/; // one liners are cool
    song.hash = Option::as_ref( &mediainfo.hash ).unwrap().to_string();
    song
}

pub fn upsert_db(songs: &Vec<std::path::PathBuf>, pgpool: &PgPool) -> Option<String> {
    use rayon::prelude::*;
    use indicatif::{ ProgressBar, ProgressStyle };

    let sty = ProgressStyle::default_bar()
        .template("{bar:40.green/yellow} {pos:>4}/{len:4} {msg}")
        .progress_chars("=> ");

    let pb = ProgressBar::new(songs.len() as u64);
    pb.set_style(sty);
    pb.tick();

    songs.par_iter().for_each(|songpath| {
        let songinfo = fill_song_info(songpath);
        Song::upsert(songinfo.clone(), &pgpool.get().unwrap()).ok();
        pb.set_message(&format!("Scanned {:?}", songinfo.filename));
        pb.inc(1);
    });
    Some(format!("updated songs"))
}

fn main() {
    let mut mpc = mpd_connect().unwrap();
    let radiofiles = get_radiofiles(dotenv!("RADIOFILES_ROOT"));

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