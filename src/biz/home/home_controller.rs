use rocket::response::content;
use rust_wheel::model::response::api_response::ApiResponse;
use crate::service::home::home_service::{overview_query, trend_query};

#[get("/v1/dashboard/overview")]
pub fn overview() -> content::Json<String> {
    let dashboard = overview_query();
    let res = ApiResponse {
        result: dashboard,
        ..Default::default()
    };
    let response_json = serde_json::to_string(&res).unwrap();
    return content::Json(response_json);
}

#[get("/v1/trend/overview")]
pub fn trend_overview() -> content::Json<String> {
    let trends = trend_query();
    let res = ApiResponse {
        result: trends,
        ..Default::default()
    };
    let response_json = serde_json::to_string(&res).unwrap();
    return content::Json(response_json);
}




