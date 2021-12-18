use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::model::response::api_response::ApiResponse;
use rust_wheel::model::user::login_user_info::LoginUserInfo;
use crate::model::request::home::home_request::HomeRequest;
use crate::model::response::home::dashboard_response::DashboardResponse;
use crate::models::{Dashboard, Favorites};
use crate::service::home::home_service::fav_music_query;

#[get("/v1/dashboard/overview")]
pub fn overview() -> content::Json<String> {
    /*let res = ApiResponse {
        result: dashboards,
        ..Default::default()
    };
    let response_json = serde_json::to_string(&res).unwrap();*/
    print!("d");
    return content::Json("response_json".parse().unwrap());
}





