use rocket::serde::Deserialize;
use rocket::serde::Serialize;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct AdminPwdEditRequest {
    pub newPassword: String,
    pub oldPassword: String,
}