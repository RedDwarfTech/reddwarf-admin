use rocket::serde::Deserialize;
use rocket::serde::Serialize;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct AddAppRequest {
    pub userId: Option<i64>,
    pub pageNum: i64,
    pub pageSize: i64,
    pub appName: String,
    pub appId: i32
}