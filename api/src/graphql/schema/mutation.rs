use juniper::FieldResult;

use crate::graphql::schema::{SongObject, NewSongInput};
use crate::graphql::Context;

#[derive(Debug)]
pub struct Mutation {}

juniper::graphql_object!(Mutation: Context |&self| {
    field add_song(&executor, input: NewSongInput) -> FieldResult<SongObject> {
        executor.context().add_song(input)
    }
});
