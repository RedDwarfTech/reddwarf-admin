use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::box_rest_response;
use crate::model::request::home::trend_request::TrendRequest;
use crate::service::home::home_service::{overview_query, trend_query};

#[get("/v1/dashboard/overview")]
pub fn overview() -> content::Json<String> {
    let dashboard = overview_query();
    return box_rest_response(dashboard);
}

#[post("/v1/trend/overview",data = "<request>")]
pub fn trend_overview(request: Json<TrendRequest>) -> content::Json<String> {
    let trends = trend_query();
    return box_rest_response(trends);
}




