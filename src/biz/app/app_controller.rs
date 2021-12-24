use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::box_rest_response;
use crate::model::diesel::dolphin::dolphin_models::App;
use crate::model::request::app::app_request::AppRequest;
use crate::service::app::app_service::app_query;

#[post("/v1/page",data = "<request>")]
pub fn page(request: Json<AppRequest>) -> content::Json<String> {
    let apps = app_query::<Vec<App>>(&request);
    return box_rest_response(apps);
}



