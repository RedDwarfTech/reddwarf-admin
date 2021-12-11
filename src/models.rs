use rocket::serde::Serialize;
use serde::Deserialize;
use crate::schema::dashboard;

#[derive(Insertable, Serialize, Queryable, Deserialize,Default)]
#[table_name = "dashboard"]
pub struct Dashboard {
    pub id: i32,
    pub app_count: i32,
    pub user_count: i32
}