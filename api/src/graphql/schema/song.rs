use juniper::{FieldError, FieldResult};

//use crate::db::enums::System;
use crate::db::models;
use crate::db::models::song::NewSong;
use crate::db::models::Song;
use crate::graphql::Context;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(description = "A song from a video game")]
pub struct SongObject {
    pub id:        String,
    pub title:     String,
    pub track:     Option<i32>,
    pub game:      String,
    pub artist:    Option<String>,
    pub year:      i32,
    pub system:    String,
    pub is_public: bool,
    pub bitrate:   i32,
    pub duration:  i32,
    pub filesize:  i32,
    pub filename:  String,
    pub fullpath:  String,
    pub hash:      String,
}

impl From<Song> for SongObject {
    fn from(song: Song) -> Self {
        SongObject {
            id:         song.id.to_string(),
            title:      song.title.clone(),
            track:      song.track.clone(),
            game:       song.game.clone(),
            artist:     song.artist.clone(),
            year:       song.year.clone(),
            system:     song.system.clone(),
            is_public:  song.is_public.clone(),
            bitrate:    song.bitrate.clone(),
            duration:   song.duration.clone(),
            filesize:   song.filesize.clone(),
            filename:   song.filename.clone(),
            fullpath:   song.fullpath.clone(),
            hash:       song.hash.clone(),
        }
    }
}


#[derive(Debug, GraphQLInputObject)]
#[graphql(description = "A song from a video game")]
pub struct NewSongInput {
    pub title:     String,
    pub track:     Option<i32>,
    pub game:      String,
    pub artist:    Option<String>,
    pub year:      i32,
    pub system:    String,
    pub is_public: bool,
    pub bitrate:   i32,
    pub duration:  i32,
    pub filesize:  i32,
    pub filename:  String,
    pub fullpath:  String,
    pub hash:      String,
}

impl Context {
    pub fn get_song(&self, id: &str) -> FieldResult<SongObject> {
        let id: i32 = id.parse()?;
        let song = models::Song::get_song(id, &self.connection)?;
        Ok(song.into())
    }

    pub fn get_song_by_hash(&self, hash: &str) -> FieldResult<SongObject> {
        let hash: String = hash.into();
        let song = models::Song::get_song_by_hash(hash, &self.connection)?;
        Ok(song.into())
    }

    pub fn all_songs(&self) -> FieldResult<Vec<SongObject>> {
        let songs = models::Song::get_all_songs(&self.connection)?;
        let song_objects =
            songs.iter().map(|song| SongObject::from(song.clone())).collect();
        Ok(song_objects)
    }

    pub fn add_song(&self, input: NewSongInput) -> FieldResult<SongObject> {
        let new_song = NewSong {
            title:      input.title.clone(),
            track:      input.track.clone(),
            game:       input.game.clone(),
            artist:     input.artist.clone(),
            year:       input.year.clone(),
            system:     input.system.clone(),
            is_public:  input.is_public.clone(),
            bitrate:    input.bitrate.clone(),
            duration:   input.duration.clone(),
            filesize:   input.filesize.clone(),
            filename:   input.filename.clone(),
            fullpath:   input.fullpath.clone(),
            hash:       input.hash.clone(),
        };

        let song = Song::create(new_song, &self.connection)
            .map_err(|_| FieldError::from("Cannot create song"))?;
        Ok(song.into())
    }
}
