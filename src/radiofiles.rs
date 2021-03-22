extern crate metadata;
extern crate globwalk;

use std::path::PathBuf;
use std::io;

use self::globwalk::glob;
use self::metadata::MediaFileMetadata;

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

pub fn upsert_db(songs: Vec<PathBuf>) {
    for s in songs {
        let mut song = NewSong::default();
        println!("{:#?}", &s);
        let tags = get_mediainfo(&s).unwrap().tags;
        for tag in tags {
            if tag.0 == "title".to_string() {
                song.title = tag.1;
                println!("Found!: {:#?}", &song.title);
            }
        }
    };
}