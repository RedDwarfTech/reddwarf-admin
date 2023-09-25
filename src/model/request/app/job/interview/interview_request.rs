use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket_okapi::okapi::schemars::JsonSchema;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
#[allow(non_snake_case)]
pub struct InterviewRequest {
    pub pageNum: i64,
    pub pageSize: i64,
    pub company: Option<String>,
    pub city: Option<String>
}