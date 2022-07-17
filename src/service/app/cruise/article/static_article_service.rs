use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl};
use rust_wheel::config::db::config;

pub fn get_article_count_by_time(start_time: i64, end_time: i64) -> i64 {
    use crate::model::diesel::dolphin::dolphin_schema::article::dsl::*;
    let connection = config::establish_connection();
    let predicate = created_time.ge(start_time).and(created_time.le(end_time));
    let query = article
        .filter(predicate);
    let query_result = query.count().get_result(&connection);
    return query_result.unwrap_or(0);
}
