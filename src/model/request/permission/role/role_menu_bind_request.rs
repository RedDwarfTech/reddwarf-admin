use rocket::serde::Deserialize;
use rocket::serde::Serialize;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct RoleMenuBindRequest {
    pub menuId: Vec<i32>,
    pub roleId: i32
}