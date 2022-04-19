use crate::model::request::permission::role::role_menu_bind_request::RoleMenuBindRequest;
use crate::service::permission::role::role_service::edit_role_menu;
use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::box_rest_response;

use crate::model::diesel::dolphin::dolphin_models::{Role};
use crate::model::request::permission::role::role_request::RoleRequest;
use crate::model::request::user::password_request::PasswordRequest;
use crate::service::permission::role::role_service::{role_edit, role_query};

#[post("/v1/page",data = "<request>")]
pub fn page(request: Json<RoleRequest>) -> content::Json<String> {
    let roles = role_query::<Vec<Role>>(&request);
    return box_rest_response(roles);
}

#[put("/v1/role/menu",data = "<request>")]
pub fn edit_role_menu_bind(request: Json<RoleMenuBindRequest>) -> content::Json<String> {
    return edit_role_menu(&request);
}

#[post("/v1/role/edit",data = "<request>")]
pub fn edit_role(request: Json<PasswordRequest>) -> content::Json<String> {
    return role_edit(&request);
}
