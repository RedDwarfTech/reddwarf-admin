use rocket::response::content;
use rust_wheel::common::util::model_convert::box_rest_response;

use crate::model::diesel::quark::quark_models::SysDict;
use crate::service::sys::sys_dict_service::{dict_page_query, dict_query};

#[get("/v1/list")]
pub fn list() -> content::Json<String> {
    let dicts = dict_query::<Vec<SysDict>>();
    return box_rest_response(dicts);
}

#[get("/v1/page")]
pub fn page() -> content::Json<String> {
    let dicts = dict_page_query::<Vec<SysDict>>();
    return box_rest_response(dicts);
}
