use chrono::NaiveDateTime;
use rocket::serde::Serialize;
use serde::Deserialize;

use crate::model::diesel::dolphin::dolphin_schema::*;

/// why we need the custom models
/// https://stackoverflow.com/questions/70547237/what-is-the-best-way-to-handle-the-id-when-insert-record-using-rust-diesel

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
#[table_name = "products"]
pub struct ProductAdd {
    pub product_name: String,
    pub remark: String,
    pub created_time: i64,
    pub updated_time: i64,
    pub user_count: i32,
    pub online_status: i32,
    pub online_time: Option<i64>,
    pub product_abbr: String,
    pub product_id: i32,
    pub product_tag: Option<String>,
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

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[table_name = "admin_users"]
pub struct AdminUserAdd {
    pub nickname: String,
    pub avatar_url: Option<String>,
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
    pub org_id: i32
}

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[table_name = "menu_resource"]
pub struct MenuResourceAdd {
    pub name: String,
    pub res_type: i32,
    pub created_time: i64,
    pub updated_time: i64,
    pub remark: Option<String>,
    pub path: String,
    pub parent_id: i32,
    pub component: Option<String>,
    pub sort: i32,
    pub name_zh: String,
    pub tree_id_path: String,
    pub code: String
}

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[table_name = "role"]
pub struct RoleAdd {
    pub name: String,
    pub created_time: i64,
    pub updated_time: i64,
    pub remark: String,
}

#[derive(AsChangeset)]
#[table_name = "rss_sub_source"]
pub struct RssSubSourceUpdate {
    pub part_output: Option<i32>,
    pub comment_rss: Option<i32>,
    pub sub_status: Option<i32>,
}

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[table_name = "menu_resource"]
pub struct MenuResourcePath {
    pub id: i32,
    pub tree_id_path: String
}

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[table_name = "tags"]
pub struct TagAdd {
    pub tag_name: String,
    pub created_time: i64,
    pub updated_time: i64,
    pub app_id: i32,
    pub remark: Option<String>,
    pub group: i32,
    pub tag_type: String,
    pub code: String,
}