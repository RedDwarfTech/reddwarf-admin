use chrono::NaiveDateTime;
use rocket::serde::Serialize;
use serde::Deserialize;

use crate::model::diesel::dolphin::dolphin_schema::*;

#[derive(Insertable,Debug,Serialize,Deserialize,Default)]
#[table_name = "domain"]
pub struct Domain {
    pub domain_name: String,
    pub domain_url: String,
    pub created_time: i64,
    pub updated_time: i64,
    pub cron: Option<String>,
    pub next_trigger_time: Option<NaiveDateTime>,
    pub monitor_status: i32,
    pub user_id: Option<i64>,
    pub expire_date: Option<NaiveDateTime>,
    pub days_before_trigger: i32,
    pub notify_trigger_date: Option<NaiveDateTime>,
    pub expire_date_ms: Option<i64>,
}

#[derive(Insertable,Queryable,Debug,Serialize,Deserialize,Default)]
#[table_name = "apps"]
pub struct AppAdd {
    pub app_name: String,
    pub remark: Option<String>,
    pub created_time: i64,
    pub updated_time: Option<i64>,
    pub user_count: Option<i32>,
    pub online_status: Option<i32>,
    pub online_time: Option<i64>,
    pub app_abbr: String,
    pub app_id: String,
    pub app_tag: Option<String>,
}

#[derive(Insertable,Queryable,Debug,Serialize,Deserialize,Default)]
#[table_name = "interview"]
pub struct InterviewAdd {
    pub city: String,
    pub address: String,
    pub company: String,
    pub created_time: i64,
    pub updated_time: i64,
    pub status: i32,
    pub info_source: i32,
    pub salary_range: String,
    pub apply_time: i64,
    pub apply_job: String,
    pub user_id: i64,
    pub job_link: String
}

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[table_name = "role_permission"]
pub struct RolePermissionAdd {
    pub role_id: i32,
    pub permission_id: i32,
    pub created_time: i64,
    pub updated_time: i64,
    pub permission_type: i32,
}

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[table_name = "user_role"]
pub struct UserRoleAdd {
    pub user_id: i64,
    pub role_id: i32,
    pub created_time: i64,
    pub updated_time: i64,
}