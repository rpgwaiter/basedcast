//use crate::config;
use crate::db;
use crate::db::Connection;
use crate::graphql::{create_schema, Context, Schema};
use rocket::response::content;
use rocket::{Rocket, State};

itconfig::config! {
    DATABASE_URL: String,

    ROCKET {
        static BASE_URL: String => "/",
    }
}

#[rocket::get("/")]
fn graphiql() -> content::Html<String> {
    println!("Attemting to get IQL");
    let graphql_endpoint_url = config::ROCKET::BASE_URL() + "/graphql";
    juniper_rocket::graphiql_source(&graphql_endpoint_url)
}

#[rocket::get("/graphql?<request>")]
fn get_graphql_handler(
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
    db: Connection,
) -> juniper_rocket::GraphQLResponse {
    println!("Request: {:#?}", &request);
    request.execute(&schema, &Context { connection: db })
}

#[rocket::post("/graphql", data = "<request>")]
fn post_graphql_handler(
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
    db: Connection,
) -> juniper_rocket::GraphQLResponse {
    println!("Request: {:#?}", &request);
    request.execute(&schema, &Context { connection: db })
}

pub fn init() -> Rocket {
    rocket::ignite().manage(db::connect()).manage(create_schema()).mount(
        &config::ROCKET::BASE_URL(),
        rocket::routes![get_graphql_handler, post_graphql_handler],
    )
}
