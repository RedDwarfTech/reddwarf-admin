use okapi::openapi3::OpenApi;
use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::box_rest_response;
use crate::model::request::app::dict::word::glossary_add_request::GlossaryAddRequest;
use crate::model::request::app::dict::word::glossary_request::GlossaryRequest;
use crate::service::app::dict::word::word_service::{glossary_add, glossary_query};
use rocket_okapi::{openapi_get_routes_spec, openapi};
use rocket_okapi::settings::OpenApiSettings;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: glossary, add_glossary]
}

#[openapi(tag = "单词")]
#[post("/v1/glossary/list",data = "<request>")]
pub fn glossary(request: Json<GlossaryRequest>) -> content::RawJson<String> {
    let dicts = glossary_query(&request);
    return box_rest_response(dicts);
}

#[openapi(tag = "单词")]
#[post("/v1/glossary/add",data = "<request>")]
pub fn add_glossary(request: Json<GlossaryAddRequest>) -> content::RawJson<String> {
    glossary_add(&request);
    return box_rest_response("ok");
}

