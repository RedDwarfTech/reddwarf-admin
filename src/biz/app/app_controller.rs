use rocket::response::content;
use rocket::serde::json::Json;
use std::default::Default;
use rust_wheel::model::response::api_response::ApiResponse;
use rust_wheel::model::response::pagination_response::PaginationResponse;
use crate::model::request::app::app_request::AppRequest;
use crate::model::request::app::music::fav::fav_music_request::FavMusicRequest;
use crate::model::response::app::music::fav::fav_music_response::FavMusicResponse;
use crate::models::{Favorites, Music};
use crate::service::app::app_service::app_query;
use crate::service::app::music::music_service::get_music_by_source_ids;

#[post("/v1/page",data = "<request>")]
pub fn page(request: Json<AppRequest>) -> content::Json<String> {
    let fav_musics = app_query::<Vec<Favorites>>(&request);
    let source_ids: Vec<String> = fav_musics.list
        .iter()
        .map(|item| item.source_id.to_string())
        .collect();
    let musics = get_music_by_source_ids::<Music>(source_ids);
    let mut result_list:Vec<FavMusicResponse> = Vec::new();
    for fav in &fav_musics.list{
        let filtered_music:Vec<_> = musics.iter()
            .filter(|item| item.source_id == fav.source_id)
            .map(|item|FavMusicResponse::new(fav,item.clone()))
            .collect();
        result_list.push(filtered_music[0].clone());
    }
    let resp = PaginationResponse{
        pagination: fav_musics.pagination,
        list: result_list
    };
    let res = ApiResponse {
        result: resp,
        ..Default::default()
    };
    let response_json = serde_json::to_string(&res).unwrap();
    return content::Json(response_json);
}



