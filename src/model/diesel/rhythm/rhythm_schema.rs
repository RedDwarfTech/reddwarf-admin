table! {
    favorites (id) {
        id -> Int8,
        song_id -> Nullable<Int8>,
        created_time -> Int8,
        updated_time -> Int8,
        user_id -> Int8,
        source_id -> Varchar,
        like_status -> Int4,
        source -> Int4,
        playlist_id -> Int8,
        play_count -> Int4,
        fetched_download_url -> Nullable<Int4>,
        downloaded -> Nullable<Int4>,
    }
}
