use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::box_rest_response;
use crate::model::diesel::dolphin::dolphin_models::Interview;

use crate::model::request::app::job::interview::interview_request::InterviewRequest;
use crate::service::app::job::interview::interview_service::interview_query;

#[post("/v1/page",data = "<request>")]
pub fn page(request: Json<InterviewRequest>) -> content::Json<String> {
    let interviews = interview_query::<Vec<Interview>>(&request);
    return box_rest_response(interviews);
}