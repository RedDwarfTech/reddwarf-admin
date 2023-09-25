use okapi::openapi3::OpenApi;
use rocket::response::content;
use rocket::serde::json::Json;
use rocket_okapi::{openapi_get_routes_spec, openapi};
use rocket_okapi::settings::OpenApiSettings;
use rust_wheel::common::wrapper::rocket_http_resp::box_rest_response;
use rust_wheel::model::response::pagination_response::PaginationResponse;
use crate::model::request::app::music::fav::fav_music_request::FavMusicRequest;
use crate::model::response::app::music::fav::fav_music_response::FavMusicResponse;
use crate::models::{Favorites, Music};
use crate::service::app::music::fav::fav_music_service::fav_music_query;
use crate::service::app::music::music_service::get_music_by_source_ids;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: page]
}

#[openapi(tag = "应用")]
#[post("/v1/page",data = "<request>")]
pub fn page(request: Json<FavMusicRequest>) -> content::RawJson<String> {
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
            .map(|item|FavMusicResponse::new(fav,item.clone()))
            .collect();
        result_list.push(filtered_music[0].clone());
    }
    let resp = PaginationResponse{
        pagination: fav_musics.pagination,
        list: result_list
    };
    return box_rest_response(resp);
}
