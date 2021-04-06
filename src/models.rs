use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::pg::upsert::on_constraint;

use schema::songs;
use schema::songs::dsl::songs as all_songs;

#[derive(Queryable, QueryableByName, Identifiable, AsChangeset, PartialEq, Debug)]
#[table_name = "songs"]
pub struct Song {
    pub id:        i32,
    pub title:     String,
    pub track:     Option<i32>,
    pub game:      Option<String>,
    pub artist:    Option<String>,
    pub year:      i32,
    pub system:    Option<String>,
    pub is_public: bool,
    pub bitrate:   i32,
    pub duration:  i32,
    pub filesize:  i32,
    pub filename:  String,
    pub fullpath:  String,
    pub hash:      String, //use UUID at some point
}

#[derive(Debug, Insertable, Default)]
#[table_name = "songs"]
pub struct NewSong {
    pub title:     String,
    pub track:     Option<i32>,
    pub game:      Option<String>,
    pub artist:    Option<String>,
    pub year:      i32,
    pub system:    Option<String>,
    pub is_public: bool,
    pub bitrate:   i32,
    pub duration:  i32,
    pub filesize:  i32,
    pub filename:  String,
    pub fullpath:  String,
    pub hash:      String,
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
        use schema::songs::dsl::{   title as t, track as tk, game as g, artist as a,
                                    year as y,
                                    system as s, bitrate as br, duration as d,
                                    filesize as fs, filename as n, fullpath as f, is_public as p,
                                    hash as h };
        let NewSong {
            title,
            track,
            game,
            artist,
            year,
            system,
            bitrate,
            duration,
            filesize,
            filename,
            fullpath,
            is_public,
            hash
        } = song;

        diesel::update(all_songs.find(id))
        .set((
            t.eq(title),
            tk.eq(track),
            g.eq(game.unwrap()),
            a.eq(artist.unwrap()),
            y.eq(year),
            s.eq(system.unwrap()),
            br.eq(bitrate),
            d.eq(duration),
            fs.eq(filesize),
            n.eq(filename),
            f.eq(fullpath),
            p.eq(is_public),
            h.eq(hash)
        ))
        .get_result::<Song>(conn)
        .is_ok()
    }

    pub fn upsert(song: NewSong, conn: &PgConnection) -> bool {
        diesel::insert_into(songs::table)
            .values(&song)
            .on_conflict(on_constraint("song_hash"))
            .do_nothing()
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
