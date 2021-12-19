use std::borrow::Borrow;
use rocket::response::content;
use rocket::serde::json::Json;
use std::default::Default;
use rust_wheel::model::response::api_response::ApiResponse;
use rust_wheel::model::user::login_user_info::LoginUserInfo;
use crate::model::request::app::music::fav::fav_music_request::FavMusicRequest;
use crate::model::request::home::home_request::HomeRequest;
use crate::model::response::app::music::fav::fav_music_response::FavMusicResponse;
use crate::model::response::home::dashboard_response::DashboardResponse;
use crate::models::{Dashboard, Favorites, Music};
use crate::service::app::music::fav::fav_music_service::fav_music_query;
use crate::service::app::music::music_service::get_music_by_sourceIds;

#[post("/v1/page",data = "<request>")]
pub fn page(request: Json<FavMusicRequest>) -> content::Json<String> {
    let fav_musics = fav_music_query::<Vec<Favorites>>(request);
    let sourceIds: Vec<String> = fav_musics.list
        .iter()
        .map(|item| item.source_id.to_string())
        .collect();
    let musics = get_music_by_sourceIds::<Music>(sourceIds);
    let mut result_list:Vec<FavMusicResponse> = Vec::new();
    for fav in &fav_musics.list{
        let mut filtered_music:Vec<_> = musics.iter()
            .filter(|item| item.source_id == fav.source_id)
            .map(|item|FavMusicResponse{
                id: (&fav.id.to_string()).parse().unwrap(),
                song_id: None,
                created_time: 0,
                updated_time: 0,
                user_id: 0,
                source_id: fav.source_id.to_string(),
                like_status: 0,
                source: 0,
                playlist_id: 0,
                play_count: 0,
                fetched_download_url: None,
                downloaded: None,
                music: item.clone()
            })
            .collect();
        result_list.push(filtered_music[0].clone());
    }
    let res = ApiResponse {
        result: result_list,
        ..Default::default()
    };
    let response_json = serde_json::to_string(&res).unwrap();
    return content::Json(response_json);
}


fn take<T>(mut vec: Vec<T>, index: usize) -> Option<T> {
    if index < vec.len() {
        Some(vec.swap_remove(index))
    } else {
        None
    }
}


