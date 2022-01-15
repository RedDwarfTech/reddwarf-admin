// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]

use rocket::serde::Serialize;
use serde::Deserialize;
use crate::model::diesel::dolphin::dolphin_schema::*;

use chrono::NaiveDateTime;
use chrono::DateTime;
use chrono::offset::Utc;
#[derive(Insertable,Queryable,Debug,Serialize,Deserialize,Default)]
#[table_name = "admin_users"]
pub struct AdminUser {
    pub id: i64,
    pub nickname: Option<String>,
    pub avatar_url: Option<String>,
    pub phone: Option<String>,
    pub updated_time: Option<i64>,
    pub created_time: Option<i64>,
    pub salt: Option<String>,
    pub pwd: Option<String>,
    pub sex: Option<i32>,
    pub level_type: Option<String>,
    pub phone_region: Option<String>,
    pub country_code: Option<i32>,
    pub user_status: Option<String>,
}

#[derive(Insertable,Queryable,Debug,Serialize,Deserialize,Default)]
#[table_name = "apps"]
pub struct App {
    pub id: i32,
    pub app_name: String,
    pub remark: Option<String>,
    pub created_time: i64,
    pub updated_time: Option<i64>,
    pub user_count: Option<i32>,
    pub online_status: Option<i32>,
    pub online_time: Option<i64>,
    pub app_abbr: String,
    pub app_id: i32,
    pub app_tag: Option<String>,
}

#[derive(Insertable,Queryable,Debug,Serialize,Deserialize,Default)]
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

#[derive(Insertable,Queryable,Debug,Serialize,Deserialize,Default)]
#[table_name = "article_content"]
pub struct ArticleContent {
    pub id: i64,
    pub article_id: i64,
    pub articleContent: String,
}

#[derive(Insertable,Queryable,Debug,Serialize,Deserialize,Default)]
#[table_name = "dashboard"]
pub struct Dashboard {
    pub id: i32,
    pub app_count: i32,
    pub user_count: i32,
}

#[derive(Insertable,Queryable,Debug,Serialize,Deserialize,Default)]
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

#[derive(Insertable,Queryable,Debug,Serialize,Deserialize,Default)]
#[table_name = "rss_sub_source"]
pub struct RssSubSource {
    pub id: i64,
    pub sub_url: String,
    pub created_time: i64,
    pub updated_time: i64,
    pub sub_status: i16,
    pub rss_type: String,
    pub standard_type: String,
    pub standard_version: String,
    pub cron: String,
    pub trigger_count: i32,
    pub next_trigger_time: Option<NaiveDateTime>,
    pub sub_name: String,
    pub last_trigger_time: Option<DateTime<Utc>>,
    pub tags: Option<Vec<i32>>,
    pub source_url: Option<String>,
    pub sub_type: Option<String>,
    pub intro: Option<String>,
    pub remark: Option<String>,
    pub title_hash: Option<String>,
    pub failed_count: i32,
    pub lang: Option<String>,
    pub frequency_month: Option<i32>,
    pub reputation: Option<i32>,
    pub rep_lastest_refresh_time: Option<i64>,
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
}

#[derive(Insertable,Queryable,Debug,Serialize,Deserialize,Default)]
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

#[derive(Insertable,Queryable,Debug,Serialize,Deserialize,Default)]
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
    pub app_id: i32,
    pub register_time: i64,
    pub apple_iap_product_id: Option<String>,
    pub auto_renew_product_expire_time_ms: Option<i64>,
}

