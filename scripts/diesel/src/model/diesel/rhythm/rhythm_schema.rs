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

table! {
    playlist (id) {
        id -> Int8,
        creator -> Int8,
        name -> Varchar,
        cover_url -> Varchar,
        description -> Nullable<Varchar>,
        subscribed -> Nullable<Int4>,
        subscribed_count -> Nullable<Int8>,
        comment_count -> Nullable<Int8>,
        share_count -> Nullable<Int4>,
        play_count -> Nullable<Int4>,
        source -> Int4,
        track_update_time -> Int8,
        created_time -> Int8,
        updated_time -> Int8,
        track_count -> Int4,
        playlist_type -> Int4,
    }
}

table! {
    songs (id) {
        id -> Int8,
        name -> Varchar,
        artists -> Varchar,
        album_id -> Int8,
        publishtime -> Int8,
        status -> Int4,
        duration -> Int4,
        source_id -> Varchar,
        source -> Int4,
        created_time -> Int8,
        updated_time -> Int8,
        album -> Varchar,
        fetched_download_url -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    favorites,
    playlist,
    songs,
);
