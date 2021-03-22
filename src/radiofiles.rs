extern crate metadata;
extern crate globwalk;

use std::path::PathBuf;
use std::io;

use self::globwalk::glob;
use self::metadata::MediaFileMetadata;

use std::convert::TryFrom;
use std::convert::TryInto;

use models::NewSong;

pub fn get_radiofiles(root: &str) -> Vec<PathBuf> {
    glob(&format!("{}/**/*.mp3", &root))
        .unwrap()
        .map(|x| x.unwrap().path().to_path_buf())
        .collect()
}

pub fn get_mediainfo(file: &PathBuf) -> Result<MediaFileMetadata, io::Error>{
    let build_media_file_metadata = |file: &PathBuf| -> io::Result<MediaFileMetadata> {
        let mut meta = MediaFileMetadata::new(&file)?;
        meta.include_checksum(true)?
            .include_tags(true);
        Ok(meta)
    };
    build_media_file_metadata(&file)
}

pub fn upsert_db(songs: Vec<PathBuf>) -> Option<String> {
    for s in &songs {
        let mut song = NewSong::default();
        let mediainfo = get_mediainfo(&s).unwrap();

        song.filesize = i32::try_from(mediainfo.file_size).unwrap();

        if let Some(bitrate) = mediainfo._bit_rate {
            song.bitrate = bitrate.try_into().unwrap();
        };

        let tags = get_mediainfo(&s).unwrap().tags; // holds a bunch of info in a Vec<String, String>
        for tag in tags {
            match tag.0.as_str() {
                "artist" => song.artist = Some(tag.1),
                "title"  => song.title = tag.1,
                "track"  => song.track = Some(tag.1.parse::<i32>().unwrap()),
                _ => println!("Unhandled tag: {:?}", tag),
            }
        }
    }
    Some(format!("updated {} songs", songs.iter().count()))
}