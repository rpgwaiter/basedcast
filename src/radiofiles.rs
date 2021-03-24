extern crate metadata;
extern crate globwalk;
extern crate regex;
extern crate uuid;

use std::path::PathBuf;
use std::io;
use std::env;

use self::globwalk::glob;
use self::metadata::MediaFileMetadata;

use std::convert::TryFrom;

use self::regex::Regex;

use models::NewSong;
use diesel::pg::upsert::*;

pub fn get_radiofiles(root: &str) -> Vec<PathBuf> {
    glob(&format!("{}/**/*.mp3", &root))
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

    let root = &env::var("RADIOFILES_ROOT").expect("Please set RADIOFILES_URL in your .env");
    let postprefix = &file.to_str().unwrap().trim_start_matches(&format!("{}/", root));

    let mut splits = postprefix.split(|c| c == '/');
    let system = splits.next().unwrap();
    
    println!("System = {:#?}", &system);
    let ginput = splits.next().unwrap();
    let gex = Regex::new(r"(?P<game>.*)\((?P<year>\d{4})\)").unwrap(); //extract game and year from folder
    let rxout = gex.captures(&ginput).unwrap();
    
    (system.to_string(),
    rxout["game"].to_string(), 
    rxout["year"].parse().unwrap())
}

fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}

fn truncate_in_place(s: &mut String, max_chars: usize) {
    s.truncate(truncate(&s, max_chars).len())
}

fn parse_tags(tags: &str) -> (String, String, String) {
    let artist = ""; let title = ""; let track = "";
    for tag in tags {
        match tag.0.as_str() {
            "artist" => artist = Some(tag.1),
            "title"  => title = tag.1,
            "track"  => track = Some( tag.1.parse::<i32>().unwrap() ),
            _ => (),
        }
    } (artist, title, track)
}

pub fn upsert_db(songs: Vec<PathBuf>) -> Option<String> {
    for s in songs {
        let mut song = NewSong::default();
        let mediainfo = get_mediainfo(&s).unwrap();
        let tags = parse_tags(get_mediainfo(&s).unwrap().tags); // holds a bunch of info in a Vec<String, String>
        let parsed = parse_path(&s); // grabs (system, game, year)

        // ## MODEL ##
        song.title = Option::as_ref(&mediainfo.title).unwrap().to_string();
        song.track = &tags.2;
        song.game = Some(parsed.1); //does this need to be optional? 
        song.artist = Some(&tags.0);
        song.year = parsed.2;
        song.system = Some(parsed.0);
        song.is_public = true;
        song.bitrate =  if let Some(b) = mediainfo._bit_rate { b as i32 } else { 0 };
        song.duration = if let Some(d) = mediainfo._duration { d as i32 } else { 0 };
        song.filesize = i32::try_from(mediainfo.file_size).unwrap();
        song.filename = mediainfo.file_name;
        song.fullpath = mediainfo.path;
        song.hash = uuid::Uuid::parse_str(truncate(Option::as_ref(&mediainfo.hash).unwrap(), 32)).unwrap() /*as diesel::pg::types::sql_types::Uuid*/; // one liners are cool
        // ## END MODEL ##

        //println!("{:#?}", &mediainfo);
        println!("{:#?}", &song);
        song.upsert()
    }
    Some(format!("updated songs"))
}