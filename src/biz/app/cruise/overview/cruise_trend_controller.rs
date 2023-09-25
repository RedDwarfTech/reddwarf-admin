use crate::model::request::app::cruise::overview::cruise_overview_request::CruiseOverviewRequest;
use crate::model::response::app::trend_response::TrendResponse;
use crate::service::app::cruise::overview::cruise_overview_service::cruise_trend_query;
use okapi::openapi3::OpenApi;
use rocket::serde::json::Json;
use rocket_okapi::settings::OpenApiSettings;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use rust_wheel::common::util::model_convert::map_entity;
use rust_wheel::common::wrapper::rocket_http_resp::box_type_rest_response;
use rust_wheel::model::response::api_response::ApiResponse;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: list]
}

#[openapi(tag = "Cruise趋势")]
#[post("/v1/list", data = "<request>")]
pub fn list(request: Json<CruiseOverviewRequest>) -> Json<ApiResponse<Vec<TrendResponse>>> {
    let trends = cruise_trend_query(&request);
    let trend_resp: Vec<TrendResponse> = map_entity(trends);
    return Json::from(box_type_rest_response(trend_resp));
}
