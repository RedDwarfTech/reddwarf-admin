use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::box_rest_response;

use crate::model::request::app::job::interview::interview_request::InterviewRequest;

#[post("/v1/page",data = "<request>")]
pub fn page(request: Json<InterviewRequest>) -> content::Json<String> {
    return box_rest_response("apps");
}