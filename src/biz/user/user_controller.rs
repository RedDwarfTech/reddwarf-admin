use okapi::openapi3::OpenApi;
use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::wrapper::rocket_http_resp::box_rest_response;
use rust_wheel::model::user::login_user_info::LoginUserInfo;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_okapi::settings::OpenApiSettings;
use crate::model::diesel::dolphin::dolphin_models::User;
use crate::model::request::user::password_request::PasswordRequest;
use crate::model::request::user::user_request::UserRequest;
use crate::service::user::user_service::{password_edit, user_query};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: page, edit_pwd]
}

#[openapi(tag = "应用")]
#[post("/v1/page",data = "<request>")]
pub fn page(request: Json<UserRequest>) -> content::RawJson<String> {
    let users = user_query::<Vec<User>>(&request);
    return box_rest_response(users);
}

#[openapi(tag = "应用")]
#[post("/v1/pwd/edit",data = "<request>")]
pub fn edit_pwd(request: Json<PasswordRequest>,login_user_info: LoginUserInfo) -> content::RawJson<String> {
    return password_edit(&request, login_user_info);
}
