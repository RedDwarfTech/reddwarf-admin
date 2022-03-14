use rocket::response::content;
use rust_wheel::common::util::model_convert::box_rest_response;

use crate::model::diesel::quark::quark_models::SysDict;
use crate::service::sys::sys_dict_service::dict_query;

#[get("/v1/list")]
pub fn page() -> content::Json<String> {
    let dicts = dict_query::<Vec<SysDict>>();
    return box_rest_response(dicts);
}



