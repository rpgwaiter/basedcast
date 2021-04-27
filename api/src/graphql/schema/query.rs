use juniper::FieldResult;

use crate::graphql::schema::SongObject;
use crate::graphql::Context;

#[derive(Debug)]
pub struct Query {}

juniper::graphql_object!(Query: Context |&self| {
    field apiVersion() -> &str {
        "1.0"
    }

    field song(&executor, id: String) -> FieldResult<SongObject> {
        executor.context().get_song(&id)
    }

    field song_by_hash(&executor, hash: String) -> FieldResult<SongObject> {
        executor.context().get_song_by_hash(&hash)
    }

    field allSongs(&executor) -> FieldResult<Vec<SongObject>> {
        executor.context().all_songs()
    }
});
