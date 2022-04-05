use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use rust_wheel::config::db::config;

pub fn channel_fav_count(req_channel_id: &i64) -> i64 {
    use crate::model::diesel::dolphin::dolphin_schema::article_favorites::dsl::*;
    let connection = config::establish_connection();
    let predicate = crate::model::diesel::dolphin::dolphin_schema::article_favorites::channel_id.eq(req_channel_id)
        .and(crate::model::diesel::dolphin::dolphin_schema::article_favorites::upvote_status.eq(1));
    let query = article_favorites
        .filter(predicate);
    // https://stackoverflow.com/questions/71634411/how-to-do-a-count-query-when-using-rust-diesel
    let query_result = query.count().get_result(&connection);
    return query_result.unwrap_or(0);
}

