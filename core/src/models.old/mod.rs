pub mod song;


pub fn show_query<T>(query: &T)
where
    T: diesel::query_builder::QueryFragment<diesel::pg::Pg>,
{
    dbg!(diesel::debug_query::<diesel::pg::Pg, _>(&query));
}

use std::sync::Arc;
use crate::db::PgPooledConnection;

pub struct Context {
    pub conn: Arc<PgPooledConnection>,
}

impl juniper::Context for Context {}

pub fn create_context(pg_pool: PgPooledConnection) -> Context {
    Context {
        conn: Arc::new(pg_pool),
    }
}