mod song;
mod mutation;
mod query;

pub use song::*;
use mutation::*;
use query::*;

pub type Schema = juniper::RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {})
}
