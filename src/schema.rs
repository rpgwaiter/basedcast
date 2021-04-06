table! {
    songs (id) {
        id -> Int4,
        title -> Text,
        track -> Nullable<Int4>,
        game -> Nullable<Text>,
        artist -> Nullable<Text>,
        year -> Int4,
        system -> Nullable<Text>,
        is_public -> Bool,
        bitrate -> Int4,
        duration -> Int4,
        filesize -> Int4,
        filename -> Text,
        fullpath -> Text,
        hash -> Text,
    }
}
