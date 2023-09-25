use okapi::openapi3::OpenApi;
use rocket::response::content;
use rocket::serde::json::Json;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_okapi::settings::OpenApiSettings;
use rust_wheel::common::wrapper::rocket_http_resp::box_rest_response;
use crate::model::diesel::dolphin::dolphin_models::{MenuResource, Role};
use crate::model::request::permission::menu::role_menu_request::RoleMenuRequest;
use crate::model::request::permission::role::role_add_request::RoleAddRequest;
use crate::model::request::permission::role::role_menu_bind_request::RoleMenuBindRequest;
use crate::model::request::permission::role::role_request::RoleRequest;
use crate::model::request::user::password_request::PasswordRequest;
use crate::model::response::permission::menu::menu_response::MenuResponse;
use crate::model::response::permission::menu::menu_response_wrapper::MenuResponseWrapper;
use crate::service::permission::menu::menu_service::menu_query_full_tree;
use crate::service::permission::role::role_service::{edit_role_menu, role_add, role_query_list};
use crate::service::permission::role::role_service::{role_edit, role_query};
use crate::service::permission::user::admin_user_service::role_menu_list;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: page, list, edit_role_menu_bind, get_role_menu_tree, edit_role, add_role]
}

#[openapi(tag = "角色")]
#[post("/v1/page",data = "<request>")]
pub fn page(request: Json<RoleRequest>) -> content::RawJson<String> {
    let roles = role_query::<Vec<Role>>(&request);
    return box_rest_response(roles);
}

#[openapi(tag = "角色")]
#[get("/v1/list")]
pub fn list() -> content::RawJson<String> {
    let roles = role_query_list();
    return box_rest_response(roles);
}

#[openapi(tag = "角色")]
#[put("/v1/role/menu",data = "<request>")]
pub fn edit_role_menu_bind(request: Json<RoleMenuBindRequest>) -> content::RawJson<String> {
    return edit_role_menu(&request);
}

#[openapi(tag = "角色")]
#[post("/v1/role/menu",data = "<request>")]
pub fn get_role_menu_tree(request: Json<RoleMenuRequest>) -> content::RawJson<String> {
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

#[openapi(tag = "角色")]
#[post("/v1/role/edit",data = "<request>")]
pub fn edit_role(request: Json<PasswordRequest>) -> content::RawJson<String> {
    return role_edit(&request);
}

#[openapi(tag = "角色")]
#[put("/v1/role/add",data = "<request>")]
pub fn add_role(request: Json<RoleAddRequest>) -> content::RawJson<String> {
    return role_add(&request);
}