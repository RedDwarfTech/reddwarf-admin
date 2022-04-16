use crate::service::permission::menu::menu_service::menu_query;
use crate::service::permission::menu::menu_service::menu_edit;
use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::box_rest_response;

use crate::model::diesel::dolphin::dolphin_models::{Role};
use crate::model::request::permission::role::role_request::RoleRequest;
use crate::model::request::user::password_request::PasswordRequest;

#[post("/v1/page",data = "<request>")]
pub fn page(request: Json<RoleRequest>) -> content::Json<String> {
    let roles = menu_query::<Vec<MenuResource>>(&request);
    return box_rest_response(roles);
}

#[post("/v1/menu/edit",data = "<request>")]
pub fn edit_menu(request: Json<PasswordRequest>) -> content::Json<String> {
    return menu_edit(&request);
}
