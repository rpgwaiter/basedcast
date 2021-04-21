// use actix_web::{ HttpRequest, HttpResponse };
// use actix_web::web;

// use crate::models::song::SongList;
// use crate::db::{ PgPool, PgPooledConnection };

// fn pg_pool_handler(pool: web::Data<PgPool>) -> Result<PgPooledConnection, HttpResponse> {
//     pool
//     .get()
//     .map_err(|e| {
//         HttpResponse::InternalServerError().json(e.to_string())
//     })
// }

// pub fn index(_req: HttpRequest, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
//     let pg_pool = pg_pool_handler(pool)?;
//     Ok(HttpResponse::Ok().json(SongList::list(&pg_pool)))
// }

// pub fn create(new_song: web::Json<Song>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
//     let pg_pool = pg_pool_handler(pool)?;
//     new_song
//         .create(&pg_pool)
//         .map(|song| HttpResponse::Ok().json(song))
//         .map_err(|e| {
//             HttpResponse::InternalServerError().json(e.to_string())
//         })
// }

// use crate::models::song::Song;

// pub fn show(id: web::Path<i32>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
//     let pg_pool = pg_pool_handler(pool)?;
//     Song::find(&id, &pg_pool)
//         .map(|song| HttpResponse::Ok().json(song))
//         .map_err(|e| {
//             HttpResponse::InternalServerError().json(e.to_string())
//         })
// }

// pub fn destroy(id: web::Path<i32>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
//     let pg_pool = pg_pool_handler(pool)?;
//     Song::destroy(&id, &pg_pool)
//         .map(|_| HttpResponse::Ok().json(()))
//         .map_err(|e| {
//             HttpResponse::InternalServerError().json(e.to_string())
//         })
// }

// pub fn update(id: web::Path<i32>, new_song: web::Json<Song>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
//     let pg_pool = pg_pool_handler(pool)?;
//     Song::update(&id, &new_song, &pg_pool)
//         .map(|_| HttpResponse::Ok().json(()))
//         .map_err(|e| {
//             HttpResponse::InternalServerError().json(e.to_string())
//         })
//}