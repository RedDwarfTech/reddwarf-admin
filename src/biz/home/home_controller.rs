use okapi::openapi3::OpenApi;
use rocket::response::content;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_okapi::settings::OpenApiSettings;
use rust_wheel::common::wrapper::rocket_http_resp::box_rest_response;
use crate::service::home::home_service::{overview_query, trend_query};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: trend_overview, overview]
}

/// # 查询概览信息
///
/// 查询概览信息
#[openapi(tag = "首页")]
#[get("/v1/dashboard/overview")]
pub fn overview() -> content::RawJson<String> {
    let dashboard = overview_query();
    return box_rest_response(dashboard);
}

/// # 查询趋势
///
/// 查询趋势
#[openapi(tag = "首页")]
#[get("/v1/trend/overview")]
pub fn trend_overview() -> content::RawJson<String> {
    let trends = trend_query();
    return box_rest_response(trends);
}
