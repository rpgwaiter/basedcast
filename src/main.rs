#[macro_use]
extern crate diesel;
extern crate dotenv;

#[macro_use]
extern crate dotenv_codegen;

mod schema;
mod models;
mod mpdctl;
mod radiofiles;

pub mod db;
pub mod handlers;

extern crate actix;
extern crate actix_web;

use actix_web::{App, HttpServer, web};
use db::establish_connection;

#[actix_rt::main]
async fn main() -> Result<(), std::io::Error> {
    let mut mpc = mpdctl::init().unwrap();

    println!("Status: {:#?}", &mpc.status().unwrap().state);

    let sys = actix::System::new();

    HttpServer::new(move
    || App::new()
        .data(establish_connection())
        .service(
            web::resource("/songs")
                .route(web::get().to(handlers::songs::index))
                //.route(web::post().to(handlers::songs::create))
        )
        // .service(
        //     web::resource("/songs/{id}")
        //         .route(web::get().to(handlers::songs::show))
        //         // .route(web::delete().to_async(handlers::songs::destroy))
        //         // .route(web::patch().to_async(handlers::songs::update))
        // )
    )
    .bind("127.0.0.1:8088").unwrap()
    .start();

    println!("Started http server: 127.0.0.1:8088");
    let _ = sys.run();
    
}