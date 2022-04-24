use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::box_rest_response;
use rust_wheel::model::user::login_user_info::LoginUserInfo;

use crate::model::diesel::dolphin::dolphin_models::AdminUser;
use crate::model::request::user::add_user_request::AddUserRequest;
use crate::model::request::user::password_request::PasswordRequest;
use crate::model::request::user::user_request::UserRequest;
use crate::model::request::user::user_role_request::UserRoleRequest;
use crate::model::response::permission::menu::dynamic_menu_response::DynamicMenuResponse;
use crate::service::permission::user::admin_user_service::{add_admin_user, admin_password_edit, admin_user_menus, save_user_roles_impl, user_roles};
use crate::service::permission::user::admin_user_service::admin_user_query;

#[post("/v1/page",data = "<request>")]
pub fn page(request: Json<UserRequest>) -> content::Json<String> {
    let users = admin_user_query::<Vec<AdminUser>>(&request);
    return box_rest_response(users);
}

#[post("/v1/pwd/edit",data = "<request>")]
pub fn edit_pwd(request: Json<PasswordRequest>) -> content::Json<String> {
    return admin_password_edit(&request);
}

#[get("/v1/menus")]
pub fn get_user_menu(login_user_info: LoginUserInfo) -> content::Json<String> {
    let menu_vec:Vec<DynamicMenuResponse> = admin_user_menus(login_user_info);
    return box_rest_response(menu_vec);
}

#[get("/v1/roles?<user_id>")]
pub fn get_user_roles(user_id: i64) -> content::Json<String> {
    let user_roles = user_roles(user_id);
    return box_rest_response(user_roles);
}

#[put("/v1/save_roles",data = "<request>")]
pub fn save_user_roles(request:Json<UserRoleRequest>) -> content::Json<String> {
    return save_user_roles_impl(request);
}

#[put("/v1/add",data = "<request>")]
pub fn add(request:Json<AddUserRequest>) -> content::Json<String> {
    return add_admin_user(request);
}