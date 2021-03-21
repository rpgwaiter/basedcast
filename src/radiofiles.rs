extern crate walkdir;
extern crate id3;
extern crate globwalk;

use self::walkdir::WalkDir;
use std::path::PathBuf;
use std::env;
use self::id3::Tag;
use self::globwalk::glob;

use std::fs;

pub fn get_radiofiles() {
    let radiofiles_root = env::var("RADIOFILES_ROOT").expect("Please set RADIOFILES_URL in your .env");

    // let list: Vec<_> = WalkDir::new(&radiofiles_root)
    //                     .into_iter()
    //                     .filter_map(|f| f.ok()) // filter out errors
    //                     .map(|f| f.path().to_owned()) // take the path and take ownership
    //                     .collect(); // collect into vector

    let list: Vec<_> = glob(&format!("{}/**/*.mp3", &radiofiles_root))
                        .filter_map(Result::ok)
                        .collect();
                        // .into_iter()
                        // //.filter_map(|f| f.ok()) // filter out errors
                        // .map(|f| f.display()) // take the path and take ownership
                        // .collect(); // collect into vector

    for i in &list {
        get_mediainfo(i);
    }

    return list;
}

pub fn get_mediainfo(path: &str) {
    println!("{:?}", path);
    if let name = path.file_name() {
        let tag = Tag::read_from_path(path).unwrap();
        if let Some(title) = tag.title() {
            println!("title: {}", title);
        }
    }

    // let song = models::NewSong {
    //     title:     &institle,
    //     game:      Some(&insgame),
    //     system:    Some(&inssystem),
    //     is_public: &insis_public,
    //     bitrate:   &insbitrate,
    //     duration:  &insduration,
    //     filesize:  &insfilesize,
    //     filename:  &insfilename,
    //     fullpath:  &insfullpath,
    // };
}