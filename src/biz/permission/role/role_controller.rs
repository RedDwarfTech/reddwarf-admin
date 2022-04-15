use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::box_rest_response;

use crate::model::diesel::dolphin::dolphin_models::User;
use crate::model::request::permission::role::role_request::RoleRequest;
use crate::model::request::user::password_request::PasswordRequest;
use crate::model::request::user::user_request::UserRequest;
use crate::service::permission::role::role_service::{role_edit, role_query};

#[post("/v1/page",data = "<request>")]
pub fn page(request: Json<RoleRequest>) -> content::Json<String> {
    let users = role_query::<Vec<User>>(&request);
    return box_rest_response(users);
}

#[post("/v1/role/edit",data = "<request>")]
pub fn edit_role(request: Json<PasswordRequest>) -> content::Json<String> {
    return role_edit(&request);
}
