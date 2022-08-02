use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl};
use rust_wheel::common::util::time_util::{get_current_millisecond, get_minus_day_millisecond};
use rust_wheel::config::db::config;

use crate::model::diesel::dolphin::dolphin_models::RssSubSource;

pub fn get_refresh_channels() -> Vec<RssSubSource> {
    use crate::model::diesel::dolphin::dolphin_schema::rss_sub_source::dsl::*;
    let connection = config::establish_connection();
    let yesterday_of_curry = get_minus_day_millisecond(-1);
    let query = rss_sub_source
        .filter(rep_latest_refresh_time.lt(yesterday_of_curry))
        .order(rep_latest_refresh_time.asc())
        .limit(20);
    let query_result = query.load::<RssSubSource>(&connection).expect("load rss source failed");
    return query_result;
}

pub fn get_low_quality_channels() -> Vec<RssSubSource> {
    use crate::model::diesel::dolphin::dolphin_schema::rss_sub_source::dsl::*;
    let connection = config::establish_connection();
    let predicate = sub_status.eq(-3).and(article_count.gt(0));
    let query = rss_sub_source
        .filter(predicate)
        .order(rep_latest_refresh_time.asc())
        .limit(2);
    let query_result = query.load::<RssSubSource>(&connection).expect("load rss source failed");
    return query_result;
}

pub fn get_refresh_channels_for_article() -> Vec<RssSubSource> {
    use crate::model::diesel::dolphin::dolphin_schema::rss_sub_source::dsl::*;
    let connection = config::establish_connection();
    let yesterday_of_curry = get_minus_day_millisecond(-2);
    let predicate = article_count_latest_refresh_time.lt(yesterday_of_curry)
        .and(sub_status.eq(1));
    let query = rss_sub_source
        .filter(predicate)
        .order(article_count_latest_refresh_time.asc())
        .limit(20);
    let query_result = query.load::<RssSubSource>(&connection).expect("load rss source failed");
    return query_result;
}

pub fn update_channel_reputation(new_reputation: i64,req_channel_id: i64) {
    use crate::model::diesel::dolphin::dolphin_schema::rss_sub_source::dsl::*;
    let connection = config::establish_connection();
    let predicate = crate::model::diesel::dolphin::dolphin_schema::rss_sub_source::id.eq(req_channel_id);
    let current_time = get_current_millisecond();
    diesel::update(rss_sub_source.filter(predicate))
        .set((reputation.eq(new_reputation),rep_latest_refresh_time.eq(current_time)))
        .get_result::<RssSubSource>(&connection)
        .expect("unable to update channel reputation");
}

pub fn update_channel_article_count(new_count: i64,req_channel_id: i64) {
    use crate::model::diesel::dolphin::dolphin_schema::rss_sub_source::dsl::*;
    let connection = config::establish_connection();
    let predicate = crate::model::diesel::dolphin::dolphin_schema::rss_sub_source::id.eq(req_channel_id);
    let current_time = get_current_millisecond();
    diesel::update(rss_sub_source.filter(predicate))
        .set((article_count.eq(new_count),article_count_latest_refresh_time.eq(current_time)))
        .get_result::<RssSubSource>(&connection)
        .expect("unable to update channel article count");
}