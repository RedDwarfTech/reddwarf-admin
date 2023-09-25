use log::info;
use okapi::openapi3::OpenApi;
use rocket::response::content;
use rocket::serde::json::Json;
use rocket_okapi::settings::OpenApiSettings;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use rust_wheel::common::wrapper::rocket_http_resp::box_rest_response;
use rust_wheel::model::user::login_user_info::LoginUserInfo;
use crate::model::diesel::dolphin::dolphin_models::Interview;
use crate::model::request::app::job::interview::add_interview_request::AddInterviewRequest;
use crate::model::request::app::job::interview::edit_interview_request::EditInterviewRequest;
use crate::model::request::app::job::interview::interview_request::InterviewRequest;
use crate::service::app::job::interview::interview_service::{add_interview, interview_query, update_interview};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: page, add, update]
}

#[openapi(tag = "面试")]
#[post("/v1/page",data = "<request>")]
pub fn page(request: Json<InterviewRequest>, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let interviews = interview_query::<Vec<Interview>>(request.0,login_user_info);
    return box_rest_response(interviews);
}

#[openapi(tag = "面试")]
#[post("/v1/add",data = "<request>")]
pub fn add(request: Json<AddInterviewRequest>,login_user_info: LoginUserInfo) -> content::RawJson<String> {
    info!("user id:{}" , login_user_info.userId);
    let interviews = add_interview(&request,login_user_info);
    return box_rest_response(interviews);
}

#[openapi(tag = "面试")]
#[post("/v1/update",data = "<request>")]
pub fn update(request: Json<EditInterviewRequest>) -> content::RawJson<String> {
    let interviews = update_interview(&request);
    return box_rest_response(interviews);
}