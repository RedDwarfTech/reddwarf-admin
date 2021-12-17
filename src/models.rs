use rocket::serde::Serialize;
use serde::Deserialize;
use crate::model::diesel::dolphin::dolphin_schema::dashboard;
use crate::model::diesel::rhythm::rhythm_schema::favorites;

#[derive(Serialize, Queryable, Deserialize,Default)]
// #[table_name = "dashboard"]
pub struct Dashboard {
    pub id: i32,
    pub app_count: i32,
    pub user_count: i32
}

#[derive( Serialize, Queryable, Deserialize,Default)]
// #[table_name = "favorites"]
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