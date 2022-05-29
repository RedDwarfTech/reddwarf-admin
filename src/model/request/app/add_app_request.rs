use rocket::serde::Deserialize;
use rocket::serde::Serialize;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct AddAppRequest {
    pub appName: String,
    pub appAbbr: String,
    pub remark: String,
    pub productId: i32
}