use rocket::serde::Deserialize;
use rocket::serde::Serialize;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
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