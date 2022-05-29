// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]

use rocket::serde::Serialize;
use serde::Deserialize;
use crate::model::diesel::dolphin::dolphin_schema::*;

use chrono::NaiveDateTime;
use chrono::DateTime;
use chrono::offset::Utc;
#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[table_name = "admin_users"]
pub struct AdminUser {
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
    pub phone_region: String,
    pub country_code: i32,
    pub user_status: i32,
    pub user_name: String,
    pub org_id: i32,
}

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[table_name = "app_repo"]
pub struct AppRepo {
    pub id: i64,
    pub app_name: String,
    pub release_datetime: i64,
    pub created_time: i64,
    pub updated_time: i64,
    pub linux_run: i32,
    pub windows_run: i32,
    pub macos_run: i32,
    pub intro: String,
    pub license: i32,
    pub official_url: Option<String>,
    pub source_url: Option<String>,
    pub author: Option<String>,
}

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[table_name = "apps"]
pub struct App {
    pub id: i32,
    pub app_name: String,
    pub remark: String,
    pub created_time: i64,
    pub updated_time: i64,
    pub user_count: i32,
    pub online_status: i32,
    pub online_time: Option<i64>,
    pub app_abbr: String,
    pub app_id: String,
    pub app_tag: Option<String>,
    pub auth_mode: i16,
    pub product_id: i32,
}

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[table_name = "article"]
pub struct Article {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub author: String,
    pub guid: String,
    pub created_time: i64,
    pub updated_time: i64,
    pub link: Option<String>,
    pub pub_time: Option<DateTime<Utc>>,
    pub sub_source_id: i64,
    pub cover_image: Option<String>,
    pub channel_reputation: i32,
    pub editor_pick: Option<i32>,
}

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[table_name = "article_content"]
pub struct ArticleContent {
    pub id: i64,
    pub article_id: i64,
    pub content: String,
}

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[table_name = "article_favorites"]
pub struct ArticleFavorite {
    pub user_id: i64,
    pub article_id: i64,
    pub created_time: i64,
    pub updated_time: i64,
    pub fav_status: i32,
    pub upvote_status: i32,
    pub channel_id: i64,
    pub read_status: i32,
    pub read_time: Option<i64>,
    pub id: i64,
}

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[table_name = "dashboard"]
pub struct Dashboard {
    pub id: i32,
    pub app_count: i32,
    pub user_count: i32,
}

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[table_name = "domain"]
pub struct Domain {
    pub id: i64,
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

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[table_name = "interview"]
pub struct Interview {
    pub id: i64,
    pub city: String,
    pub company: String,
    pub created_time: i64,
    pub updated_time: i64,
    pub address: String,
    pub status: i32,
    pub interview_time: Option<i64>,
    pub info_source: i32,
    pub salary_range: Option<String>,
    pub apply_time: Option<i64>,
    pub apply_job: Option<String>,
    pub user_id: Option<i64>,
    pub job_link: Option<String>,
}

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[table_name = "menu_resource"]
pub struct MenuResource {
    pub id: i32,
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
    pub code: String,
}

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[table_name = "org"]
pub struct Org {
    pub id: i32,
    pub parent_id: i32,
    pub created_time: i64,
    pub updated_time: i64,
    pub org_name: String,
    pub sort: i32,
    pub tree_id_path: String,
}

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[table_name = "products"]
pub struct Product {
    pub id: i32,
    pub product_name: String,
    pub remark: Option<String>,
    pub created_time: i64,
    pub updated_time: i64,
    pub user_count: i32,
    pub online_status: i32,
    pub online_time: Option<i64>,
    pub product_abbr: String,
    pub product_id: i32,
    pub product_tag: Option<String>,
}

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[table_name = "role"]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub status: i32,
    pub updated_time: i64,
    pub created_time: i64,
    pub remark: String,
    pub is_deleted: i32,
}

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[table_name = "role_permission"]
pub struct RolePermission {
    pub id: i64,
    pub role_id: i32,
    pub permission_id: i32,
    pub created_time: i64,
    pub updated_time: i64,
    pub permission_type: i32,
}

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[table_name = "rss_sub_source"]
pub struct RssSubSource {
    pub id: i64,
    pub sub_url: String,
    pub created_time: i64,
    pub updated_time: i64,
    pub sub_status: i32,
    pub rss_type: String,
    pub standard_type: String,
    pub standard_version: String,
    pub cron: String,
    pub trigger_count: i32,
    pub next_trigger_time: Option<NaiveDateTime>,
    pub sub_name: String,
    pub last_trigger_time: Option<DateTime<Utc>>,
    pub source_url: Option<String>,
    pub sub_type: Option<String>,
    pub intro: Option<String>,
    pub remark: Option<String>,
    pub title_hash: Option<String>,
    pub failed_count: i32,
    pub lang: Option<String>,
    pub frequency_month: Option<i32>,
    pub reputation: Option<i64>,
    pub rep_latest_refresh_time: Option<i64>,
    pub scrapy_take_time: Option<i32>,
    pub follower: Option<i64>,
    pub censor_status: Option<i32>,
    pub etag: Option<String>,
    pub last_modified: Option<String>,
    pub editor_pick: Option<i32>,
    pub fav_icon_url: Option<String>,
    pub dynamic_interval: i32,
    pub local_icon_url: Option<String>,
    pub creator: i64,
    pub tags: serde_json::Value,
    pub article_count: i64,
    pub article_count_latest_refresh_time: i64,
    pub comment_rss: i32,
    pub part_output: i32,
}

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[table_name = "tags"]
pub struct Tag {
    pub id: i32,
    pub tag_name: String,
    pub created_time: i64,
    pub updated_time: i64,
    pub app_id: i32,
    pub remark: Option<String>,
    pub group: i32,
    pub tag_type: String,
    pub code: String,
}

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[table_name = "trend"]
pub struct Trend {
    pub id: i64,
    pub trend_item: i32,
    pub app_id: i32,
    pub created_time: i64,
    pub updated_time: i64,
    pub statistic_time: i64,
    pub incre_num: i32,
}

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[table_name = "user_devices"]
pub struct UserDevice {
    pub id: i64,
    pub device_id: String,
    pub device_name: Option<String>,
    pub user_id: i64,
    pub created_time: i64,
    pub updated_time: i64,
}

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[table_name = "user_role"]
pub struct UserRole {
    pub id: i32,
    pub user_id: i64,
    pub role_id: i32,
    pub created_time: i64,
    pub updated_time: i64,
}

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[table_name = "users"]
pub struct User {
    pub id: i64,
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
    pub last_login_time: Option<i64>,
    pub first_login_time: Option<i64>,
    pub app_id: String,
    pub register_time: i64,
    pub apple_iap_product_id: Option<String>,
    pub auto_renew_product_expire_time_ms: Option<i64>,
    pub product_id: Option<i32>,
}

