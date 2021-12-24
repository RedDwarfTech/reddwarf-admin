use rocket::serde::Deserialize;
use rocket::serde::Serialize;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct PasswordRequest {
    pub loginType: i32,
    pub newPassword: String,
    pub oldPassword: String,
    pub userName: String,
}