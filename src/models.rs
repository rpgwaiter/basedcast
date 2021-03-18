use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use schema::media;
use schema::media::dsl::media as all_media;

#[derive(Queryable, PartialEq, Debug)]
pub struct Media {
    pub id: i32,
    pub title: String,
    pub system: String,
    pub bitrate: i32,
    pub duration: i32,
    pub filesize: i32,
    pub filename: String,
    pub fullpath: String,
    pub is_public: bool
}

#[derive(Insertable)]
#[table_name = "media"]
pub struct NewMedia {
    pub id: i32,
    pub title: String,
    pub system: String,
    pub bitrate: i32,
    pub duration: i32,
    pub filesize: i32,
    pub filename: String,
    pub fullpath: String,
    pub is_public: bool
}

impl Media {
    pub fn show(id: i32, conn: &PgConnection) -> Vec<Media> {
        all_media
            .find(id)
            .load::<Media>(conn)
            .expect("Error loading media")
    }

    pub fn all(conn: &PgConnection) -> Vec<Media> {
        all_media
            .order(media::id.desc())
            .load::<Media>(conn)
            .expect("Error loading all media")
    }
    pub fn update_by_id(id: i32, conn: &PgConnection, media: NewMedia) -> bool {
        use schema::media::dsl::{ title as t, system as s, bitrate as br, duration as d,
            filesize as fs, filename as n, fullpath as f, is_public as p };
        let NewMedia {
            id,
            title,
            system,
            bitrate,
            duration,
            filesize,
            filename,
            fullpath,
            is_public
        } = media;

        diesel::update(all_media.find(id))
        .set((
            t.eq(title),
            s.eq(system),
            br.eq(bitrate),
            d.eq(duration),
            fs.eq(filesize),
            n.eq(filename),
            f.eq(fullpath),
            p.eq(is_public)
        ))
        .get_result::<Media>(conn)
        .is_ok()
    }

    pub fn insert(media: NewMedia, conn: &PgConnection) -> bool {
        diesel::insert_into(media::table)
            .values(&media)
            .execute(conn)
            .is_ok()
    }

    pub fn delete_by_id(id: i32, conn: &PgConnection) -> bool {
        if Media::show(id, conn).is_empty() {
            return false;
        }
        diesel::delete(all_media.find(id)).execute(conn).is_ok()
    }

    pub fn all_on_system(system: String, conn: &PgConnection) -> Vec<Media> {
        all_media
            .filter(media::system.eq(system))
            .load::<Media>(conn)
            .expect("Error loading media")
    }
}
