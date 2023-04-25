use okapi::openapi3::OpenApi;
use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::box_rest_response;

use crate::model::diesel::dolphin::dolphin_models::AppRepo;
use crate::model::request::app::add_app_request::AddAppRequest;
use crate::model::request::app::app_request::AppRequest;
use crate::model::request::app::gallery::repo_app_request::RepoAppRequest;
use crate::service::app::gallery::repo_app_service::{repo_app_create, repo_app_detail, repo_app_edit, repo_app_query};
use rocket_okapi::{openapi_get_routes_spec, openapi};
use rocket_okapi::settings::OpenApiSettings;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: page, add, edit, detail]
}

#[openapi(tag = "应用")]
#[post("/v1/page",data = "<request>")]
pub fn page(request: Json<AppRequest>) -> content::RawJson<String> {
    let apps = repo_app_query::<Vec<AppRepo>>(&request);
    return box_rest_response(apps);
}

#[openapi(tag = "应用")]
#[post("/v1/add",data = "<request>")]
pub fn add(request: Json<AddAppRequest>) -> content::RawJson<String> {
    repo_app_create(&request);
    return box_rest_response("ok");
}

#[openapi(tag = "应用")]
#[put("/v1/edit",data = "<request>")]
pub fn edit(request: Json<RepoAppRequest>) -> content::RawJson<String> {
    repo_app_edit(&request);
    return box_rest_response("ok");
}

#[openapi(tag = "应用")]
#[put("/v1/detail/<id>")]
pub fn detail(id: i32) -> content::RawJson<String> {
    let app = repo_app_detail(id);
    return box_rest_response(app);
}