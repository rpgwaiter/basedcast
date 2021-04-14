extern crate metadata;
extern crate globwalk;
extern crate regex;
extern crate rayon;
extern crate r2d2_postgres;
extern crate r2d2;
extern crate linya;

use std::path::PathBuf;
use std::convert::TryFrom;
use std::{io, env};

use self::metadata::MediaFileMetadata;
use crate::db::establish_connection;

use crate::models::song::NewSong;

pub fn get_radiofiles(root: &str) -> Vec<PathBuf> {
    self::globwalk::glob(&format!("{}/**/*.mp3", &root))
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

fn fill_song_info(s: &PathBuf) -> NewSong { 
    let mut song = NewSong::default();

    let mediainfo = get_mediainfo(&s).unwrap();
    let tags = parse_tags(get_mediainfo(&s).unwrap().tags);
    let parsed = parse_path(&s); // grabs (system, game, year)

    song.title = Option::as_ref(&mediainfo.title).unwrap().to_string();
    song.track = Some(tags.2);
    song.game = Some(parsed.1); //does this need to be optional? 
    song.artist = Some(tags.0);
    song.year = parsed.2;
    song.system = Some(parsed.0);
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

pub fn upsert_db(songs: &Vec<PathBuf>) -> Option<String> {
    use std::sync::{Arc, Mutex};
    use self::linya::{Bar, Progress};
    use self::rayon::prelude::*;

    let progress = Arc::new(Mutex::new(Progress::new()));

    let bar: Bar = progress.lock().unwrap().bar(
        songs.iter().count(),
        format!("Scanning {} songs...", songs.iter().count())
    );

    songs.par_iter().for_each_with(progress, |p, n| {
        NewSong::upsert(&fill_song_info(n), &establish_connection().get().unwrap()).ok();
        p.lock().unwrap().inc_and_draw(&bar, 1);
    });

    Some(format!("updated songs"))
}