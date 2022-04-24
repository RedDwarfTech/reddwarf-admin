use rocket::serde::Deserialize;
use rocket::serde::Serialize;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct AddUserRequest {
    pub userName: i64,
    pub phone: String,
}