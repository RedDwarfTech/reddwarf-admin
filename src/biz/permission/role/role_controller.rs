use crate::model::request::permission::role::role_menu_bind_request::RoleMenuBindRequest;
use crate::service::permission::role::role_service::{edit_role_menu, role_query_list};
use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::box_rest_response;
use rust_wheel::model::user::login_user_info::LoginUserInfo;

use crate::model::diesel::dolphin::dolphin_models::{MenuResource, Role};
use crate::model::request::permission::menu::menu_request::MenuRequest;
use crate::model::request::permission::menu::role_menu_request::RoleMenuRequest;
use crate::model::request::permission::role::role_request::RoleRequest;
use crate::model::request::user::password_request::PasswordRequest;
use crate::model::response::permission::menu::menu_response::MenuResponse;
use crate::model::response::permission::menu::menu_response_wrapper::MenuResponseWrapper;
use crate::service::permission::menu::menu_service::menu_query_full_tree;
use crate::service::permission::role::role_service::{role_edit, role_query};
use crate::service::permission::user::admin_user_service::{admin_user_menus_list, role_menu_list};

#[post("/v1/page",data = "<request>")]
pub fn page(request: Json<RoleRequest>) -> content::Json<String> {
    let roles = role_query::<Vec<Role>>(&request);
    return box_rest_response(roles);
}

#[get("/v1/list")]
pub fn list() -> content::Json<String> {
    let roles = role_query_list();
    return box_rest_response(roles);
}

#[put("/v1/role/menu",data = "<request>")]
pub fn edit_role_menu_bind(request: Json<RoleMenuBindRequest>) -> content::Json<String> {
    return edit_role_menu(&request);
}

#[post("/v1/role/menu",data = "<request>")]
pub fn get_role_menu_tree(request: Json<RoleMenuRequest>) -> content::Json<String> {
    let menu_responses:Vec<MenuResponse> = menu_query_full_tree::<Vec<MenuResource>>(request.parentId);
    let menu_vec:Vec<MenuResource> = role_menu_list(request.roleId);
    let ids:Vec<String> = menu_vec.iter()
        .map(|item| item.tree_id_path.to_string())
        .collect();
    let menu_wrapper = MenuResponseWrapper{
        menus: menu_responses,
        checked_keys: ids
    };
    return box_rest_response(menu_wrapper);
}

#[post("/v1/role/edit",data = "<request>")]
pub fn edit_role(request: Json<PasswordRequest>) -> content::Json<String> {
    return role_edit(&request);
}
