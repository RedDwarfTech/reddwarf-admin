use rocket::serde::Deserialize;
use rocket::serde::Serialize;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct EditAppRequest {
    pub appName: String,
    pub appAbbr: String,
    pub appId: i32,
    pub remark: String
}