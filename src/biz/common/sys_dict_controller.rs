use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::box_rest_response;
use crate::model::diesel::dolphin::dolphin_models::App;
use crate::model::diesel::quark::quark_models::SysDict;
use crate::model::request::app::app_request::AppRequest;
use crate::service::app::app_service::app_query;
use crate::service::sys::sys_dict_service::dict_query;

#[get("/v1/list")]
pub fn page() -> content::Json<String> {
    let dicts = dict_query::<Vec<SysDict>>();
    return box_rest_response(dicts);
}



