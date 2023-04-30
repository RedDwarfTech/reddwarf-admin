
use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;

use crate::model::diesel::dolphin::dolphin_models::{App};

///
/// https://stackoverflow.com/questions/73405960/the-trait-jsonschema-is-not-implemented-for-chronodatetimeutc
///
#[derive( Serialize, Queryable, Deserialize,Default, Clone, JsonSchema)]
pub struct AppResponse {
    pub id: i32,
    pub app_name: String,
    pub remark: String,
    pub product_name: String,
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

impl AppResponse {
    pub(crate) fn new(article: &App) -> Self {
        Self {
            id: 0,
            app_name: "".to_string(),
            created_time: article.created_time,
            updated_time: article.updated_time,
            user_count: 0,
            online_status: 0,
            online_time: None,
            app_abbr: "".to_string(),
            app_id: "".to_string(),
            app_tag: None,
            auth_mode: 0,
            remark: "".to_string(),
            product_id: 0,
            product_name: "".to_string(),
        }
    }
}

impl From<&App> for AppResponse {
    fn from(p: &App) -> Self {
        Self {
            id: p.id,
            app_name: p.app_name.to_string(),
            remark: p.remark.to_string(),
            product_name: "".to_string(),
            created_time: p.created_time,
            updated_time: p.updated_time,
            user_count: p.user_count,
            online_status: p.online_status,
            online_time: None,
            app_abbr: p.app_abbr.to_string(),
            app_id: p.app_id.to_string(),
            app_tag: None,
            auth_mode: p.auth_mode,
            product_id: p.product_id,
        }
    }
}
