use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use crate::models::Music;

#[derive( Serialize, Queryable, Deserialize,Default, Clone)]
pub struct FavMusicResponse{
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
    pub downloaded: Option<i32>,
    pub music: Music
}