use crate::service::permission::menu::menu_service::menu_query_full_tree;
use crate::model::request::permission::menu::menu_request::MenuRequest;
use crate::service::permission::menu::menu_service::menu_query_tree;
use crate::service::permission::menu::menu_service::menu_edit;
use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::box_rest_response;

use crate::model::diesel::dolphin::dolphin_models::{MenuResource};
use crate::model::request::user::password_request::PasswordRequest;

#[post("/v1/page",data = "<request>")]
pub fn page_tree(request: Json<MenuRequest>) -> content::Json<String> {
    let roles = menu_query_tree::<Vec<MenuResource>>(&request);
    return box_rest_response(roles);
}

#[post("/v1/tree",data = "<request>")]
pub fn menu_tree(request: Json<MenuRequest>) -> content::Json<String> {
    let roles = menu_query_full_tree::<Vec<MenuResource>>(&request);
    return box_rest_response(roles);
}

#[post("/v1/menu/edit",data = "<request>")]
pub fn edit_menu(request: Json<PasswordRequest>) -> content::Json<String> {
    return menu_edit(&request);
}
