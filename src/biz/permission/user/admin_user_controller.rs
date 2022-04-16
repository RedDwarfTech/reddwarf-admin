use crate::model::diesel::dolphin::dolphin_models::AdminUser;
use crate::service::permission::user::admin_user_service::admin_password_edit;
use crate::service::permission::user::admin_user_service::admin_user_query;
use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::box_rest_response;

use crate::model::request::user::password_request::PasswordRequest;
use crate::model::request::user::user_request::UserRequest;

#[post("/v1/page",data = "<request>")]
pub fn page(request: Json<UserRequest>) -> content::Json<String> {
    let users = admin_user_query::<Vec<AdminUser>>(&request);
    return box_rest_response(users);
}

#[post("/v1/pwd/edit",data = "<request>")]
pub fn edit_pwd(request: Json<PasswordRequest>) -> content::Json<String> {
    return admin_password_edit(&request);
}
