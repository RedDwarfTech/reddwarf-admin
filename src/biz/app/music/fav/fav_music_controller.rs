use std::borrow::Borrow;
use rocket::response::content;
use rocket::serde::json::Json;
use std::default::Default;
use rust_wheel::model::response::api_response::ApiResponse;
use rust_wheel::model::response::pagination::Pagination;
use rust_wheel::model::response::pagination_response::PaginationResponse;
use rust_wheel::model::user::login_user_info::LoginUserInfo;
use crate::model::request::app::music::fav::fav_music_request::FavMusicRequest;
use crate::model::request::home::home_request::HomeRequest;
use crate::model::response::app::music::fav::fav_music_response::FavMusicResponse;
use crate::model::response::home::dashboard_response::DashboardResponse;
use crate::models::{Dashboard, Favorites, Music};
use crate::service::app::music::fav::fav_music_service::fav_music_query;
use crate::service::app::music::music_service::get_music_by_source_ids;

#[post("/v1/page",data = "<request>")]
pub fn page(request: Json<FavMusicRequest>) -> content::Json<String> {
    let fav_musics = fav_music_query::<Vec<Favorites>>(&request);
    let source_ids: Vec<String> = fav_musics.list
        .iter()
        .map(|item| item.source_id.to_string())
        .collect();
    let musics = get_music_by_source_ids::<Music>(source_ids);
    let mut result_list:Vec<FavMusicResponse> = Vec::new();
    for fav in &fav_musics.list{
        let filtered_music:Vec<_> = musics.iter()
            .filter(|item| item.source_id == fav.source_id)
            .map(|item|FavMusicResponse::from(fav,item.clone()))
            .collect();
        result_list.push(filtered_music[0].clone());
    }
    let page_result = Pagination{
        pageNum: request.pageNum,
        pageSize: request.pageSize,
        total: 20
    };
    let resp = PaginationResponse{
        pagination: page_result,
        list: result_list
    };
    let res = ApiResponse {
        result: resp,
        ..Default::default()
    };
    let response_json = serde_json::to_string(&res).unwrap();
    return content::Json(response_json);
}



