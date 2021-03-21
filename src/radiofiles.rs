extern crate id3;
extern crate globwalk;

use std::path::Path;
use self::id3::Tag;
use self::globwalk::glob;


pub fn get_radiofiles(root: &str) -> Vec<Path> {

    let ret: Vec<Path> = glob(&format!("{}/**/*.mp3", &root))
        .unwrap()
        .map(|x| x.unwrap().path())
        .collect();

    return ret
}

pub fn get_mediainfo(path: &Path) {
    println!("{:?}", path);
        let tag = Tag::read_from_path(path).unwrap();
        if let Some(title) = tag.title() {
            println!("title: {}", title);
        }
    
}