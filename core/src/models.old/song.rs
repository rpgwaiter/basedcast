use crate::schema::songs;
use juniper::FieldResult;
use crate::models::Context;
use diesel_full_text_search::{plainto_tsquery, TsVectorExtensions};
use diesel::{QueryDsl, RunQueryDsl};


use diesel::{
    pg::Pg,
    pg::upsert::on_constraint,
    PgConnection,
};

#[derive(Serialize, Deserialize)]
pub struct SongList(pub Vec<Song>);
impl From<Vec<Song>> for SongList {
    fn from(vec: Vec<Song>) -> Self {
       SongList(vec)
    }
 }

#[derive(
    Default,
    Insertable,
    Identifiable,
    Queryable,
    Serialize,
    Deserialize,
    juniper::GraphQLObject
)]
#[table_name = "songs"]
#[graphql(description = "Song")]
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
    pub hash:      String,
}

pub type SongColumns = (
    songs::id,
    songs::title,
    songs::track,
    songs::game,
    songs::artist,
    songs::year,
    songs::system,
    songs::is_public,
    songs::bitrate,
    songs::duration,
    songs::filesize,
    songs::filename,
    songs::fullpath,
    songs::hash,
);

pub const SONG_COLUMNS: SongColumns = (
    songs::id,
    songs::title,
    songs::track,
    songs::game,
    songs::artist,
    songs::year,
    songs::system,
    songs::is_public,
    songs::bitrate,
    songs::duration,
    songs::filesize,
    songs::filename,
    songs::fullpath,
    songs::hash,
);

impl Song {
    pub fn upsert(&self, conn: &PgConnection) -> Result<Song, diesel::result::Error> {

        diesel::insert_into(songs::table)
            .values(self)
            .on_conflict(on_constraint("song_hash"))
            .do_nothing()
            .get_result(conn)
    }
    // pub fn list(connection: &PgConnection) -> SongList {
    //     use diesel::RunQueryDsl;
    //     //use diesel::QueryDsl;
    //     use crate::schema::songs::dsl::*;

    //     let result = 
    //         songs
    //             //.limit(10)
    //             .load::<Song>(connection)
    //             .expect("Error loading songs");

    //     SongList(result)
    // }
    pub fn list(
        context: &Context,
        search: String,
        limit: i32,
        //rank: f64,
    ) -> FieldResult<SongList> {
        let connection: &PgConnection = &context.conn;
        let mut query = crate::schema::songs::table.into_boxed::<Pg>();

        // if !search.is_empty() {
        //     query = query
        //         //.filter(crate::schema::songs::fullpath.matches(plainto_tsquery(search.clone())))
        //         .order((
        //             songs::id.desc(),
        //             songs::fullpath.distance(plainto_tsquery(search)),
        //         ));
        // } else {
        //     query = query.order(songs::id.desc());
        // }
        let query_songs = SongList::from(
            query
                .select(SONG_COLUMNS)
                //.filter(song_rank.le(rank))
                .limit(i64::from(limit))
                .load::<Song>(connection)?
        );

        // let products_with_prices = PriceProduct::belonging_to(&query_products)
        //     .inner_join(schema::prices::table)
        //     .load::<(PriceProduct, Price)>(connection)?
        //     .grouped_by(&query_products);
        Ok(query_songs)
    }
    


    // pub fn find(id: &i32, conn: &PgConnection) -> Result<Song, diesel::result::Error> {
    //     use diesel::QueryDsl;
    //     use diesel::RunQueryDsl;

    //     songs::table.find(id).first(conn)
    // }

    // // TODO: Add flag to delete the files or just mark as deleted in the db
    // pub fn destroy(id: &i32, conn: &PgConnection) -> Result<(), diesel::result::Error> {
    //     use diesel::QueryDsl;
    //     use diesel::RunQueryDsl;
    //     use crate::schema::songs::dsl;

    //     diesel::delete(dsl::songs.find(id)).execute(conn)?;
    //     Ok(())
    // }

    // pub fn update(id: &i32, new_song: &NewSong, conn: &PgConnection) ->
    //  Result<(), diesel::result::Error> {
    //     use diesel::QueryDsl;
    //     use diesel::RunQueryDsl;
    //     use crate::schema::songs::dsl;

    //     diesel::update(dsl::songs.find(id))
    //         .set(new_song)
    //         .execute(conn)?;
    //     Ok(())
    // }
}