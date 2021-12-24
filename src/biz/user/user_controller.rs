use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::box_rest_response;
use crate::model::diesel::dolphin::dolphin_models::User;
use crate::model::request::user::password_request::PasswordRequest;
use crate::model::request::user::user_request::UserRequest;
use crate::service::user::user_service::{password_edit, user_query};

#[post("/v1/page",data = "<request>")]
pub fn page(request: Json<UserRequest>) -> content::Json<String> {
    let users = user_query::<Vec<User>>(&request);
    return box_rest_response(users);
}

#[post("/v1/pwd/edit",data = "<request>")]
pub fn edit(request: Json<PasswordRequest>) -> content::Json<String> {
    let result = password_edit(&request);
    return box_rest_response(result);
}
