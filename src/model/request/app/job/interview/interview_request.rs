use rocket::serde::Deserialize;
use rocket::serde::Serialize;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct InterviewRequest {
    pub pageNum: i64,
    pub pageSize: i64,
    pub company: Option<String>,
    pub city: Option<String>
}