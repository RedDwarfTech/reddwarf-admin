use rocket::serde::Deserialize;
use rocket::serde::Serialize;

use crate::model::request::permission::menu::role_menu_request::RoleMenuRequest;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct AddMenuRequest {
    pub code: String,
    pub name: String,
    pub parentId: i32,
}
