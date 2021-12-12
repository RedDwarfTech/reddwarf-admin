use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::model::response::api_response::ApiResponse;
use crate::model::request::user::login_request::LoginRequest;
use crate::model::response::home::dashboard_response::DashboardResponse;
use crate::models::Dashboard;
use crate::service::home::home_service::dashboard_impl;

#[post("/login",data = "<record>")]
pub fn login(record: Json<LoginRequest>) -> content::Json<String> {
    let res = ApiResponse {
        result: "ok",
        ..Default::default()
    };
    let response_json = serde_json::to_string(&res).unwrap();
    return content::Json(response_json);
}





