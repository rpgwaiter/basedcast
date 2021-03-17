use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

pub struct Media {
    pub id: i32,
    pub title: String,
    pub bitrate: String,
    pub duration: String,
    pub filesize: i32,
    pub filename: String,
    pub fullpath: String
}