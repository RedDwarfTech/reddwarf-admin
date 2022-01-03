use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::box_rest_response;
use crate::model::diesel::dict::dict_models::Dict;
use crate::model::request::app::dict::translate::translate_request::TranslateRequest;
use crate::service::app::dict::translate::translate_service::translate_query;
use crate::service::app::dict::word::word_service::glossary_query;

#[post("/v1/glossary",data = "<request>")]
pub fn glossary(request: Json<TranslateRequest>) -> content::Json<String> {
    let dicts = glossary_query(&request);
    return box_rest_response(dicts);
}



