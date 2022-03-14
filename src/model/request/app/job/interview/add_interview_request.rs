use rocket::serde::Deserialize;
use rocket::serde::Serialize;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct AddInterviewRequest {
    pub city: String,
    pub company: String,
    pub address: String
}