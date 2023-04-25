
use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;

use crate::model::diesel::dolphin::dolphin_models::{Trend};


#[derive( Serialize, Queryable, Deserialize,Default, Clone, JsonSchema)]
pub struct TrendResponse {
    pub id: i64,
    pub trend_item: i32,
    pub app_id: i32,
    pub created_time: i64,
    pub updated_time: i64,
    pub statistic_time: i64,
    pub incre_num: i32,
}

impl From<&Trend> for TrendResponse {
    fn from(p: &Trend) -> Self {
        Self {
            id: p.id,
            trend_item: p.trend_item,
            app_id: p.app_id,
            created_time: p.created_time,
            updated_time: p.updated_time,
            statistic_time: p.statistic_time,
            incre_num: p.incre_num,
        }
    }
}