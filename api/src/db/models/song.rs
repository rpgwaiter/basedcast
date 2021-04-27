use diesel;
use diesel::prelude::*;
use diesel::PgConnection;

//use crate::db::enums::System;
pub use crate::db::schema::*;

#[derive(AsChangeset, Queryable, Identifiable, Debug, Clone, PartialEq)]
#[table_name = "songs"]
pub struct Song {
    pub id:        i32,
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

#[derive(Clone, Debug, Default, Insertable)]
#[table_name = "songs"]
pub struct NewSong {
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

impl Song {
    pub fn create(
        new_song: NewSong,
        connection: &PgConnection,
    ) -> QueryResult<Song> {
        diesel::insert_into(songs::table)
            .values(&new_song)
            .get_result(connection)
    }

    pub fn upsert(new_song: NewSong, connection: &PgConnection) -> Result<Song, diesel::result::Error> {
        use diesel::pg::upsert::on_constraint;

        diesel::insert_into(songs::table)
            .values(new_song)
            .on_conflict(on_constraint("song_hash"))
            .do_nothing()
            .get_result(connection)
    }

    pub fn get_song(id: i32, connection: &PgConnection) -> QueryResult<Song> {
        songs::table.find(id).first::<Song>(connection)
    }

    pub fn get_song_by_hash(h: String, connection: &PgConnection) -> QueryResult<Song> {
        use songs::hash;
        songs::table.filter(hash.eq(h)).first::<Song>(connection)
    }

    pub fn get_all_songs(
        connection: &PgConnection,
    ) -> QueryResult<Vec<Song>> {
        songs::table.order(songs::id).load::<Song>(connection)
    }

    pub fn update(id: i32, song: Song, connection: &PgConnection) -> bool {
        diesel::update(songs::table.find(id))
            .set(&song)
            .execute(connection)
            .is_ok()
    }

    pub fn delete(id: i32, connection: &PgConnection) -> bool {
        diesel::delete(songs::table.find(id)).execute(connection).is_ok()
    }
}
