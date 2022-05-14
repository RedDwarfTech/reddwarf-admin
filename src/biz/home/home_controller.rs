use rocket::response::content;
use rust_wheel::common::util::model_convert::box_rest_response;

use crate::service::home::home_service::{overview_query, trend_query};

#[get("/v1/dashboard/overview")]
pub fn overview() -> content::RawJson<String> {
    let dashboard = overview_query();
    return box_rest_response(dashboard);
}

#[get("/v1/trend/overview")]
pub fn trend_overview() -> content::RawJson<String> {
    let trends = trend_query();
    return box_rest_response(trends);
}




