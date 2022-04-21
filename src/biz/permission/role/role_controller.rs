use crate::model::request::permission::role::role_menu_bind_request::RoleMenuBindRequest;
use crate::service::permission::role::role_service::edit_role_menu;
use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::box_rest_response;
use rust_wheel::model::user::login_user_info::LoginUserInfo;

use crate::model::diesel::dolphin::dolphin_models::{MenuResource, Role};
use crate::model::request::permission::menu::menu_request::MenuRequest;
use crate::model::request::permission::role::role_request::RoleRequest;
use crate::model::request::user::password_request::PasswordRequest;
use crate::model::response::permission::menu::dynamic_menu_response::DynamicMenuResponse;
use crate::model::response::permission::menu::menu_response::MenuResponse;
use crate::service::permission::menu::menu_service::menu_query_full_tree;
use crate::service::permission::role::role_service::{role_edit, role_query};
use crate::service::permission::user::admin_user_service::admin_user_menus;

#[post("/v1/page",data = "<request>")]
pub fn page(request: Json<RoleRequest>) -> content::Json<String> {
    let roles = role_query::<Vec<Role>>(&request);
    return box_rest_response(roles);
}

#[put("/v1/role/menu",data = "<request>")]
pub fn edit_role_menu_bind(request: Json<RoleMenuBindRequest>) -> content::Json<String> {
    return edit_role_menu(&request);
}

#[post("/v1/role/menu",data = "<request>")]
pub fn get_role_menu_tree(request: Json<MenuRequest>,login_user_info: LoginUserInfo) -> content::Json<String> {
    let mut menu_responses:Vec<MenuResponse> = menu_query_full_tree::<Vec<MenuResource>>(&request);
    let menu_vec:Vec<DynamicMenuResponse> = admin_user_menus(login_user_info);
    for menu_res in menu_responses.iter_mut() {
       let contains= menu_vec.iter().find(|&&m| m.id == menu_res.id);
        if contains.is_some(){
            menu_res.disableCheckbox = false;
        }
    }
    return box_rest_response(menu_responses);
}

#[post("/v1/role/edit",data = "<request>")]
pub fn edit_role(request: Json<PasswordRequest>) -> content::Json<String> {
    return role_edit(&request);
}
