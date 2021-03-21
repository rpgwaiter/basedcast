table! {
    songs (id) {
        id -> Int4,
        title -> Text,
        game -> Nullable<Text>,
        system -> Nullable<Text>,
        is_public -> Bool,
        bitrate -> Int4,
        duration -> Int4,
        filesize -> Int4,
        filename -> Text,
        fullpath -> Text,
    }
}
