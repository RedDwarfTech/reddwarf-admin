use rocket::serde::Deserialize;
use rocket::serde::Serialize;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct RoleMenuRequest {
    pub pageNum: i64,
    pub pageSize: i64,
    pub parentId: i32,
    pub roleId: i32
}