use okapi::openapi3::OpenApi;
use rocket::response::content;
use rocket::serde::json::Json;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_okapi::settings::OpenApiSettings;
use rust_wheel::common::util::model_convert::box_rest_response;

use crate::model::diesel::quark::quark_models::SysDict;
use crate::model::request::sys::sys_dict_request::SysDictRequest;
use crate::service::sys::sys_dict_service::{dict_page_query, dict_query};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: list, page]
}

/// # 列表查询
///
/// 列表查询
#[openapi(tag = "字典")]
#[get("/v1/list")]
pub fn list() -> content::RawJson<String> {
    let dicts = dict_query::<Vec<SysDict>>();
    return box_rest_response(dicts);
}

/// # 分页查询
///
/// 分页查询
#[openapi(tag = "字典")]
#[post("/v1/page",data = "<request>")]
pub fn page(request: Json<SysDictRequest>) -> content::RawJson<String> {
    let dicts = dict_page_query::<Vec<SysDict>>(request);
    return box_rest_response(dicts);
}

