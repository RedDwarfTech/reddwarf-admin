use okapi::openapi3::OpenApi;
use rocket::response::content;
use rocket::serde::json::Json;
use rocket_okapi::{openapi, openapi_get_routes_spec};


use rocket_okapi::settings::OpenApiSettings;
use rust_wheel::common::util::model_convert::box_rest_response;

use crate::model::diesel::dolphin::dolphin_models::Tag;
use crate::model::request::app::add_app_request::AddAppRequest;
use crate::model::request::app::app_request::AppRequest;
use crate::model::request::app::edit_app_request::EditAppRequest;
use crate::model::request::app::tag_list_request::TagListRequest;
use crate::service::app::tag_service::{tag_create, tag_detail, tag_edit, tag_query, tag_query_list};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: page, list, add, edit, detail]
}

#[openapi(tag = "标签")]
#[post("/v1/page",data = "<request>")]
pub fn page(request: Json<AppRequest>) -> content::RawJson<String> {
    let apps = tag_query::<Vec<Tag>>(&request);
    return box_rest_response(apps);
}

#[openapi(tag = "标签")]
#[post("/v1/list",data = "<request>")]
pub fn list(request: Json<TagListRequest>) -> content::RawJson<String> {
    let apps = tag_query_list(&request);
    return box_rest_response(apps);
}

#[openapi(tag = "标签")]
#[post("/v1/add",data = "<request>")]
pub fn add(request: Json<AddAppRequest>) -> content::RawJson<String> {
    tag_create(&request);
    return box_rest_response("ok");
}

#[openapi(tag = "标签")]
#[put("/v1/edit",data = "<request>")]
pub fn edit(request: Json<EditAppRequest>) -> content::RawJson<String> {
    tag_edit(&request);
    return box_rest_response("ok");
}

#[openapi(tag = "标签")]
#[put("/v1/detail/<id>")]
pub fn detail(id: i32) -> content::RawJson<String> {
    let app = tag_detail(id);
    return box_rest_response(app);
}