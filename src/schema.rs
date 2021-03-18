table! {
    media (id) {
        id -> Int4,
        title -> Varchar,
        system -> Varchar,
        is_public -> Bool,
        bitrate -> Nullable<Int4>,
        duration -> Nullable<Numeric>,
        filesize -> Int4,
        filename -> Varchar,
        fullpath -> Varchar,
    }
}
