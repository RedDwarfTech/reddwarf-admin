use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket_okapi::okapi::schemars::JsonSchema;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
#[allow(non_snake_case)]
pub struct EditInterviewRequest {
    pub id: i64,
    pub city: String,
    pub company: String,
    pub address: String,
    pub status: i32,
    pub salary_range: String,
    pub job_link: String
}