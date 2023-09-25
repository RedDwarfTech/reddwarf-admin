use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use rust_wheel::config::db::config;

use crate::{model::diesel::dolphin::dolphin_models::ArticleFavorite, common::db::database::get_conn};

/// diesel did not support group by
/// https://stackoverflow.com/questions/59197346/how-to-do-sum-in-diesel-rust
pub fn channel_fav_count(req_channel_id: &i64) -> i64 {
    use crate::model::diesel::dolphin::dolphin_schema::article_favorites::dsl::*;
    let connection = config::establish_connection();
    let predicate = channel_id.eq(req_channel_id);
    let fav_query = article_favorites
        .filter(predicate);
    // https://stackoverflow.com/questions/71634411/how-to-do-a-count-query-when-using-rust-diesel
    let favorite_result = fav_query.load::<ArticleFavorite>(&mut get_conn()).expect("load favorite failed");
    let upvote_sum = favorite_result.iter()
        .map(|item| item.upvote_status)
        .sum::<i32>();
    return upvote_sum as i64;
}

