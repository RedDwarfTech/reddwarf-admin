use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::box_rest_response;

use crate::model::diesel::dolphin::dolphin_models::App;
use crate::model::request::app::add_app_request::AddAppRequest;
use crate::model::request::app::app_request::AppRequest;
use crate::model::request::app::edit_app_request::EditAppRequest;
use crate::service::app::app_service::{app_create, app_detail, app_edit, app_query};

#[post("/v1/page",data = "<request>")]
pub fn page(request: Json<AppRequest>) -> content::RawJson<String> {
    let apps = app_query::<Vec<App>>(&request);
    return box_rest_response(apps);
}

#[post("/v1/add",data = "<request>")]
pub fn add(request: Json<AddAppRequest>) -> content::RawJson<String> {
    app_create(&request);
    return box_rest_response("ok");
}

#[patch("/v1/edit",data = "<request>")]
pub fn edit(request: Json<EditAppRequest>) -> content::RawJson<String> {
    app_edit(&request);
    return box_rest_response("ok");
}

#[put("/v1/detail/<id>")]
pub fn detail(id: i32) -> content::RawJson<String> {
    let app = app_detail(id);
    return box_rest_response(app);
}