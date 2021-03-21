use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use schema::songs;
use schema::songs::dsl::songs as all_songs;

#[derive(Queryable, Identifiable, AsChangeset, PartialEq, Debug)]
pub struct Song {
    pub id:        i32,
    pub title:     String,
    pub game:      Option<String>,
    pub system:    Option<String>,
    pub is_public: bool,
    pub bitrate:   i32,
    pub duration:  i32,
    pub filesize:  i32,
    pub filename:  String,
    pub fullpath:  String,
}

#[derive(Debug, Insertable)]
#[table_name = "songs"]
pub struct NewSong<'a> {
    pub title:     &'a String,
    pub game:      Option<&'a String>,
    pub system:    Option<&'a String>,
    pub is_public: &'a bool,
    pub bitrate:   &'a i32,
    pub duration:  &'a i32,
    pub filesize:  &'a i32,
    pub filename:  &'a String,
    pub fullpath:  &'a String,
}

impl Song {
    pub fn show(id: i32, conn: &PgConnection) -> Vec<Song> {
        all_songs
            .find(id)
            .load::<Song>(conn)
            .expect("Error loading song")
    }

    pub fn all(conn: &PgConnection) -> Vec<Song> {
        all_songs
            .order(songs::id.desc())
            .load::<Song>(conn)
            .expect("Error loading all songs")
    }
    pub fn update_by_id(id: i32, conn: &PgConnection, song: NewSong) -> bool {
        use schema::songs::dsl::{ title as t, game as g, system as s, bitrate as br, duration as d,
            filesize as fs, filename as n, fullpath as f, is_public as p };
        let NewSong {
            title,
            game,
            system,
            bitrate,
            duration,
            filesize,
            filename,
            fullpath,
            is_public
        } = song;

        diesel::update(all_songs.find(id))
        .set((
            t.eq(title),
            g.eq(game.unwrap()),
            s.eq(system.unwrap()),
            br.eq(bitrate),
            d.eq(duration),
            fs.eq(filesize),
            n.eq(filename),
            f.eq(fullpath),
            p.eq(is_public)
        ))
        .get_result::<Song>(conn)
        .is_ok()
    }

    pub fn insert(song: NewSong, conn: &PgConnection) -> bool {
        diesel::insert_into(songs::table)
            .values(&song)
            .execute(conn)
            .is_ok()
    }

    pub fn delete_by_id(id: i32, conn: &PgConnection) -> bool {
        if Song::show(id, conn).is_empty() {
            return false;
        }
        diesel::delete(all_songs.find(id)).execute(conn).is_ok()
    }

    pub fn all_on_system(system: String, conn: &PgConnection) -> Vec<Song> {
        all_songs
            .filter(songs::system.eq(system))
            .load::<Song>(conn)
            .expect("Error loading songs")
    }
}
