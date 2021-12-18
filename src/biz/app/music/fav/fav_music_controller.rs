use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::model::response::api_response::ApiResponse;
use rust_wheel::model::user::login_user_info::LoginUserInfo;
use crate::model::request::app::music::fav::fav_music_request::FavMusicRequest;
use crate::model::request::home::home_request::HomeRequest;
use crate::model::response::home::dashboard_response::DashboardResponse;
use crate::models::{Dashboard, Favorites};
use crate::service::home::home_service::fav_music_query;

#[post("/v1/fav/page",data = "<request>")]
pub fn page(request: Json<FavMusicRequest>) -> content::Json<String> {
    let fav_musics = fav_music_query::<Vec<Favorites>>(request);
    let res = ApiResponse {
        result: fav_musics,
        ..Default::default()
    };
    let response_json = serde_json::to_string(&res).unwrap();
    return content::Json(response_json);
}





