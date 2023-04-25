use okapi::openapi3::OpenApi;
use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::box_rest_response;
use crate::model::request::app::dict::translate::translate_request::TranslateRequest;
use crate::service::app::dict::translate::translate_service::translate_query;
use rocket_okapi::{openapi_get_routes_spec, openapi};
use rocket_okapi::settings::OpenApiSettings;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: trans]
}

#[openapi(tag = "翻译")]
#[post("/v1/translate",data = "<request>")]
pub fn trans(request: Json<TranslateRequest>) -> content::RawJson<String> {
    let dicts = translate_query(&request);
    return box_rest_response(dicts);
}



