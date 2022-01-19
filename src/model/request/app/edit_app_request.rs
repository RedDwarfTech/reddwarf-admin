use rocket::serde::Deserialize;
use rocket::serde::Serialize;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct EditAppRequest {
    pub remark: String,
    pub onlineStatus: i32,
    pub appId: i32
}