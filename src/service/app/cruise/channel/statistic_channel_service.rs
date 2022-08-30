use chrono::{DateTime, Duration, FixedOffset, NaiveDateTime, TimeZone, Utc};
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl};
use diesel::dsl::any;
use rust_wheel::common::util::time_util::{get_current_millisecond, get_minus_day_millisecond};
use rust_wheel::config::db::config;

use crate::model::diesel::dolphin::dolphin_models::{ArticleFavorite, RssSubSource};

pub fn get_refresh_channels() -> Vec<RssSubSource> {
    use crate::model::diesel::dolphin::dolphin_schema::article_favorites::dsl::*;
    let connection = config::establish_connection();
    let dt = Utc::now() + Duration::minutes(-5);
    let fav_query = article_favorites
        .filter(updated_time.lt(dt.timestamp_millis()));
    let favorite_result = fav_query.load::<ArticleFavorite>(&connection).expect("load favorite failed");
    let ids:Vec<i64> = favorite_result.iter()
        .map(|item| item.channel_id)
        .collect();
    return if ids.is_empty() {
        Vec::new()
    } else {
        get_recent_changed_channel(ids)
    }
}

/// 只需要更新最新有调整的频道即可
/// 不需要更新未改变的频道
///
pub fn get_recent_changed_channel(ids: Vec<i64>) -> Vec<RssSubSource>{
    use crate::model::diesel::dolphin::dolphin_schema::rss_sub_source::dsl::*;
    let connection = config::establish_connection();
    let query = rss_sub_source
        .filter(id.eq(any(ids)));
    let query_result = query.load::<RssSubSource>(&connection).expect("load rss source failed");
    return query_result;
}

pub fn get_low_quality_channels() -> Vec<RssSubSource> {
    use crate::model::diesel::dolphin::dolphin_schema::rss_sub_source::dsl::*;
    let connection = config::establish_connection();
    // https://stackoverflow.com/questions/70669873/initialize-vector-using-vec-macro-and-fill-it-with-values-from-existing-array
    let sub_status_arr:Vec<i32> = vec![-3,-6];
    let predicate = sub_status.eq(any(sub_status_arr)).and(article_count.gt(0));
    let query = rss_sub_source
        .filter(predicate)
        .order(rep_latest_refresh_time.asc())
        .limit(2);
    let query_result = query.load::<RssSubSource>(&connection).expect("load rss source failed");
    return query_result;
}

pub fn get_refresh_channels_by_time() -> Vec<RssSubSource>{
    use crate::model::diesel::dolphin::dolphin_schema::rss_sub_source::dsl::*;
    let connection = config::establish_connection();
    let trigger_time = (get_current_millisecond() - 35000)/1000;
    let time_without_zone = NaiveDateTime::from_timestamp( trigger_time ,0);
    let tz_offset = FixedOffset::east(8 * 3600);
    let dt_with_tz: DateTime<FixedOffset> = tz_offset.from_local_datetime(&time_without_zone).unwrap();
    let predicate = last_trigger_time.lt(dt_with_tz)
        .and(sub_status.eq(1));
    let query = rss_sub_source
        .filter(predicate)
        .order(article_count_latest_refresh_time.asc());
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