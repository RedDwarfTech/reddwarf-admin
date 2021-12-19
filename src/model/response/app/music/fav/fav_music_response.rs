use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use crate::models::{Favorites, Music};
use num_traits::cast::ToPrimitive;

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

impl FavMusicResponse {
    pub(crate) fn new(f: &Favorites, music: Music) -> Self {
        Self {
            id: f.id.to_i64().unwrap(),
            song_id: None,
            created_time: f.created_time.to_i64().unwrap(),
            updated_time: 0,
            user_id: 0,
            source_id: f.source_id.to_string(),
            like_status: 0,
            source: 0,
            playlist_id: 0,
            play_count: 0,
            fetched_download_url: None,
            downloaded: None,
            music,
        }
    }
}

