use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::box_rest_response;
use crate::model::diesel::dolphin::dolphin_models::App;
use crate::model::request::app::add_app_request::AddAppRequest;
use crate::model::request::app::app_request::AppRequest;
use crate::service::app::app_service::{app_create, app_query};

#[post("/v1/page",data = "<request>")]
pub fn page(request: Json<AppRequest>) -> content::Json<String> {
    let apps = app_query::<Vec<App>>(&request);
    return box_rest_response(apps);
}

#[post("/v1/add",data = "<request>")]
pub fn add(request: Json<AddAppRequest>) -> content::Json<String> {
    app_create(&request);
    return box_rest_response("ok");
}

