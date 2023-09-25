use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema, Clone)]
#[allow(non_snake_case)]
pub struct SysDictRequest {
    pub dict_type: Option<String>,
    pub pageNum: i64,
    pub pageSize: i64
}