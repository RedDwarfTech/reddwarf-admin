use rocket::serde::Deserialize;
use rocket::serde::Serialize;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct UserRoleRequest {
    pub userId: i64,
    pub roleIds: Vec<i32>
}