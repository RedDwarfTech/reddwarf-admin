use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::box_rest_response;
use crate::model::request::app::dict::translate::translate_request::TranslateRequest;
use crate::service::app::dict::translate::translate_service::translate_query;

#[post("/v1/translate",data = "<request>")]
pub fn trans(request: Json<TranslateRequest>) -> content::Json<String> {
    let dicts = translate_query(&request);
    return box_rest_response(dicts);
}



