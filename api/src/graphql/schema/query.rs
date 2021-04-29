use juniper::FieldResult;

use crate::graphql::schema::SongObject;
use crate::graphql::Context;

#[derive(Debug)]
pub struct Query {}

juniper::graphql_object!(Query: Context |&self| {
    field apiVersion() -> &str {
        "1.0"
    }

    field songById(&executor, id: String) -> FieldResult<SongObject> {
        executor.context().get_song(&id)
    }

    field songByHash(&executor, hash: String) -> FieldResult<SongObject> {
        executor.context().get_song_by_hash(&hash)
    }

    field allSongs(&executor) -> FieldResult<Vec<SongObject>> {
        executor.context().all_songs()
    }
});
