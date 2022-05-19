use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::box_rest_response;

use crate::model::diesel::quark::quark_models::SysDict;
use crate::model::request::sys::sys_dict_request::SysDictRequest;
use crate::service::sys::sys_dict_service::{dict_page_query, dict_query};

#[get("/v1/list")]
pub fn list() -> content::RawJson<String> {
    let dicts = dict_query::<Vec<SysDict>>();
    return box_rest_response(dicts);
}

#[post("/v1/page",data = "<request>")]
pub fn page(request: Json<SysDictRequest>) -> content::RawJson<String> {
    let dicts = dict_page_query::<Vec<SysDict>>(request);
    return box_rest_response(dicts);
}

