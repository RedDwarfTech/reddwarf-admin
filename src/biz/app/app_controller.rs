use rocket::response::content;
use rocket::serde::json::Json;
use std::default::Default;
use rust_wheel::model::response::api_response::ApiResponse;
use crate::model::diesel::dolphin::dolphin_models::App;
use crate::model::request::app::app_request::AppRequest;
use crate::service::app::app_service::app_query;

#[post("/v1/page",data = "<request>")]
pub fn page(request: Json<AppRequest>) -> content::Json<String> {
    let apps = app_query::<Vec<App>>(&request);
    let res = ApiResponse {
        result: apps,
        ..Default::default()
    };
    let response_json = serde_json::to_string(&res).unwrap();
    return content::Json(response_json);
}



