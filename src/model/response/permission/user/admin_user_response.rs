use rocket::serde::Deserialize;
use rocket::serde::Serialize;

#[derive(Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct AdminUserResponse {
    pub id: i64,
    pub nickname: String,
    pub avatar_url: String,
    pub phone: String,
    pub updated_time: i64,
    pub created_time: i64,
    pub salt: String,
    pub pwd: String,
    pub sex: Option<i32>,
    pub level_type: Option<String>,
    pub phone_region: Option<String>,
    pub country_code: Option<i32>,
    pub user_status: i32,
    pub user_name: String,
    pub org_id: i32,
    pub org_name: String
}
