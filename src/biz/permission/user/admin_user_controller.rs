use okapi::openapi3::OpenApi;
use rocket::response::content;
use rocket::serde::json::Json;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_okapi::settings::OpenApiSettings;
use rust_wheel::common::util::model_convert::box_rest_response;
use rust_wheel::model::user::login_user_info::LoginUserInfo;

use crate::model::diesel::dolphin::dolphin_models::AdminUser;
use crate::model::request::user::add_user_request::AddUserRequest;
use crate::model::request::user::admin_pwd_edit_request::AdminPwdEditRequest;
use crate::model::request::user::user_request::UserRequest;
use crate::model::request::user::user_role_request::UserRoleRequest;
use crate::model::response::permission::menu::dynamic_menu_response::DynamicMenuResponse;
use crate::service::permission::user::admin_user_service::{add_admin_user, admin_password_edit, admin_user_menus, save_user_roles_impl, user_roles};
use crate::service::permission::user::admin_user_service::admin_user_query;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: page, edit_pwd, get_user_menu, get_user_roles, save_user_roles, add]
}

#[openapi(tag = "后台用户")]
#[post("/v1/page",data = "<request>")]
pub fn page(request: Json<UserRequest>) -> content::RawJson<String> {
    let users = admin_user_query::<Vec<AdminUser>>(&request);
    return box_rest_response(users);
}

#[openapi(tag = "后台用户")]
#[post("/v1/pwd/edit",data = "<request>")]
pub fn edit_pwd(request: Json<AdminPwdEditRequest>,login_user_info: LoginUserInfo) -> content::RawJson<String> {
    return admin_password_edit(&request, login_user_info);
}

#[openapi(tag = "后台用户")]
#[get("/v1/menus")]
pub fn get_user_menu(login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let menu_vec:Vec<DynamicMenuResponse> = admin_user_menus(login_user_info);
    return box_rest_response(menu_vec);
}

#[openapi(tag = "后台用户")]
#[get("/v1/roles?<user_id>")]
pub fn get_user_roles(user_id: i64) -> content::RawJson<String> {
    let user_roles = user_roles(user_id);
    return box_rest_response(user_roles);
}

#[openapi(tag = "后台用户")]
#[put("/v1/save_roles",data = "<request>")]
pub fn save_user_roles(request:Json<UserRoleRequest>) -> content::RawJson<String> {
    return save_user_roles_impl(request);
}

#[openapi(tag = "后台用户")]
#[put("/v1/add",data = "<request>")]
pub fn add(request:Json<AddUserRequest>) -> content::RawJson<String> {
    return add_admin_user(request);
}