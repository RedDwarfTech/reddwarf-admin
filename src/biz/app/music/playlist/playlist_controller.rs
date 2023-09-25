use okapi::openapi3::OpenApi;
use rocket::response::content;
use rocket::serde::json::Json;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_okapi::settings::OpenApiSettings;
use rust_wheel::common::wrapper::rocket_http_resp::box_rest_response;
use crate::model::diesel::dolphin::dolphin_models::App;
use crate::model::request::app::add_app_request::AddAppRequest;
use crate::model::request::app::app_request::AppRequest;
use crate::model::request::app::edit_app_request::EditAppRequest;
use crate::service::app::app_service::{app_create, app_detail, app_edit, app_query};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: page, add, edit, detail]
}

#[openapi(tag = "歌单")]
#[post("/v1/page",data = "<request>")]
pub fn page(request: Json<AppRequest>) -> content::RawJson<String> {
    let apps = app_query::<Vec<App>>(&request);
    return box_rest_response(apps);
}

#[openapi(tag = "歌单")]
#[post("/v1/add",data = "<request>")]
pub fn add(request: Json<AddAppRequest>) -> content::RawJson<String> {
    app_create(&request);
    return box_rest_response("ok");
}

#[openapi(tag = "歌单")]
#[patch("/v1/edit",data = "<request>")]
pub fn edit(request: Json<EditAppRequest>) -> content::RawJson<String> {
    app_edit(&request);
    return box_rest_response("ok");
}

#[openapi(tag = "歌单")]
#[put("/v1/detail/<id>")]
pub fn detail(id: i32) -> content::RawJson<String> {
    let app = app_detail(id);
    return box_rest_response(app);
}