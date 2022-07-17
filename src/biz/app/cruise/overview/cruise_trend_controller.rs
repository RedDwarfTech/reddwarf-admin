use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::box_type_rest_response;
use rust_wheel::model::response::api_response::ApiResponse;

use crate::model::diesel::dolphin::dolphin_models::Trend;
use crate::model::request::app::cruise::overview::cruise_overview_request::CruiseOverviewRequest;
use crate::service::app::cruise::overview::cruise_overview_service::cruise_trend_query;

#[post("/v1/list", data = "<request>")]
pub fn list(request: Json<CruiseOverviewRequest>) -> Json<ApiResponse<Vec<Trend>>> {
    let trends = cruise_trend_query(&request);
    return Json::from(box_type_rest_response(trends));
}