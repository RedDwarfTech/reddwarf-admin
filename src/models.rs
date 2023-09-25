use rocket::serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Queryable, Deserialize,Default)]
pub struct Dashboard {
    pub id: i32,
    pub app_count: i32,
    pub user_count: i32
}

#[derive( Serialize, Queryable, Deserialize,Default)]
pub struct Favorites {
    pub id: i64,
    pub song_id: Option<i64>,
    pub created_time: i64,
    pub updated_time: i64,
    pub user_id: i64,
    pub source_id: String,
    pub like_status: i32,
    pub source: i32,
    pub playlist_id: i64,
    pub play_count: i32,
    pub fetched_download_url: Option<i32>,
    pub downloaded: Option<i32>
}


#[derive( Serialize, Queryable, Deserialize,Default, Clone)]
pub struct Music {
    pub id: i64,
    pub name: String,
    pub artists: String,
    pub album_id: i64,
    pub publishtime: i64,
    pub status: i32,
    pub duration: i32,
    pub source_id: String,
    pub source: i32,
    pub created_time: i64,
    pub updated_time:i64,
    pub album: String,
    pub fetched_download_url: i32
}