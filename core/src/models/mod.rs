pub mod song;


// pub fn show_query<T>(query: &T)
// where
//     T: diesel::query_builder::QueryFragment<diesel::pg::Pg>,
// {
//     dbg!(diesel::debug_query::<diesel::pg::Pg, _>(&query));
// }

// pub struct Context {
//     pub user_id: i32,
//     pub conn: Arc<PgPooledConnection>,
// }

// impl juniper::Context for Context {}

// pub fn create_context(logged_user_id: i32, pg_pool: PgPooledConnection) -> Context {
//     Context {
//         user_id: logged_user_id,
//         conn: Arc::new(pg_pool),
//     }
//}