use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::box_rest_response;
use crate::model::diesel::dolphin::dolphin_models::App;
use crate::model::request::app::add_app_request::AddAppRequest;
use crate::model::request::app::app_request::AppRequest;
use crate::model::request::app::edit_app_request::EditAppRequest;
use crate::model::request::app::job::interview::interview_request::InterviewRequest;
use crate::service::app::app_service::{app_create, app_detail, app_edit, app_query};

#[post("/v1/page",data = "<request>")]
pub fn page(request: Json<InterviewRequest>) -> content::Json<String> {
    return box_rest_response("apps");
}