use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use diesel::dsl::count;
use rocket::serde::json::Json;
use rust_wheel::config::db::config;

pub fn channel_fav_count(req_channel_id: &i64) -> i64 {
    use crate::model::diesel::dolphin::dolphin_schema::article_favorites::dsl::*;
    let connection = config::establish_connection();
    let query = article_favorites
        .filter(channel_id.eq(req_channel_id));
    let query_result = query.count().get_result(&connection);
    return query_result.unwrap_or(0);
}

