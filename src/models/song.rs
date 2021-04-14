use crate::schema::songs;
use diesel::{PgConnection, pg::upsert::on_constraint};
use super::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SongList(pub Vec<Song>);

#[derive(Queryable, Serialize, Deserialize)]
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
    pub hash:      String,
}

pub type SongColumns = (
    products::id,
    products::title,
    products::track,
    products::game,
    products::artist,
    products::year,
    products::system,
    products::is_public,
    products::bitrate,
    products::duration,
    products::filesize,
    products::filename,
    products::fullpath,
    products::hash,
);

pub const SONG_COLUMNS: SongColumns = (
    products::id,
    products::title,
    products::track,
    products::game,
    products::artist,
    products::year,
    products::system,
    products::is_public,
    products::bitrate,
    products::duration,
    products::filesize,
    products::filename,
    products::fullpath,
    products::hash,
);


#[allow(dead_code)]
impl SongList {
    pub fn list(connection: &PgConnection) -> Self {
        use diesel::RunQueryDsl;
        //use diesel::QueryDsl;
        use crate::schema::songs::dsl::*;

        let result = 
            songs
                //.limit(10)
                .load::<Song>(connection)
                .expect("Error loading songs");

        SongList(result)
    }
}

impl NewSong {

    pub fn upsert(&self, conn: &PgConnection) -> Result<Song, diesel::result::Error> {
        use diesel::RunQueryDsl;

        diesel::insert_into(songs::table)
            .values(self)
            .on_conflict(on_constraint("song_hash"))
            .do_nothing()
            .get_result(conn)
    }
}

impl Song {
    pub fn list(
        context: &Context,
        search: String,
        limit: i32,
        rank: f64,
    ) -> FieldResult<ListSong> {
        let connection: &PgConnection = &context.conn;
        let mut query = schema::songs::table.into_boxed::<Pg>();

        if !search.is_empty() {
            query = query
                .filter(text_searchable_song_col.matches(plainto_tsquery(search.clone())))
                .order((
                    song_rank.desc(),
                    text_searchable_song_col.distance(plainto_tsquery(search)),
                ));
        } else {
            query = query.order(song_rank.desc());
        }
        let query_song = query
            .select(SONG_COLUMNS)
            .filter(user_id.eq(context.user_id).and(song_rank.le(rank)))
            .limit(i64::from(limit))
            .load::<Song>(connection)?;

        // example for later
        // let products_with_prices = PriceProduct::belonging_to(&query_products)
        //     .inner_join(schema::prices::table)
        //     .load::<(PriceProduct, Price)>(connection)?
        //     .grouped_by(&query_products);

        // let vec_full_product = query_products
        //     .into_iter()
        //     .zip(products_with_prices)
        //     .map(|tuple_product| {
        //         let full_price_product = tuple_product
        //             .1
        //             .iter()
        //             .map(|tuple_price_product| FullPriceProduct {
        //                 price_product: tuple_price_product.0.clone(),
        //                 price: tuple_price_product.1.clone(),
        //             })
        //             .collect();
        //         FullSong {
        //             product: tuple_product.0.clone(),
        //             price_products: full_price_product,
        //         }
        //     })
        //     .collect();
        
        Ok(ListSong {
            data: vec_full_product,
        })
    }
    pub fn find(id: &i32, conn: &PgConnection) -> Result<Song, diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;

        songs::table.find(id).first(conn)
    }

    // TODO: Add flag to delete the files or just mark as deleted in the db
    pub fn destroy(id: &i32, conn: &PgConnection) -> Result<(), diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use crate::schema::songs::dsl;

        diesel::delete(dsl::songs.find(id)).execute(conn)?;
        Ok(())
    }

    pub fn update(id: &i32, new_song: &NewSong, conn: &PgConnection) ->
     Result<(), diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use crate::schema::songs::dsl;

        diesel::update(dsl::songs.find(id))
            .set(new_song)
            .execute(conn)?;
        Ok(())
    }
}