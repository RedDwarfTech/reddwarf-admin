use rocket::serde::Deserialize;
use rocket::serde::Serialize;

#[derive(Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct DashboardResponse {
    pub id: i32,
    pub appCount: i32,
    pub userCount: i32,
}

impl Default for DashboardResponse {
    fn default() -> Self {
        DashboardResponse {
            id: 0,
            appCount: 0,
            userCount: 0
        }
    }
}
