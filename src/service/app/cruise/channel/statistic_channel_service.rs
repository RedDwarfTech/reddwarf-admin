use diesel::{BoxableExpression, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use diesel::sql_types::Bool;
use rocket::serde::json::Json;
use rust_wheel::common::query::pagination::PaginateForQueryFragment;
use rust_wheel::common::util::model_convert::map_pagination_res;
use rust_wheel::config::db::config;
use rust_wheel::model::response::pagination_response::PaginationResponse;

use crate::model::diesel::dolphin::dolphin_models::RssSubSource;
use crate::model::request::app::cruise::channel::channel_request::ChannelRequest;

type DB = diesel::pg::Pg;

pub fn get_refresh_channels() -> Vec<RssSubSource> {
    use crate::model::diesel::dolphin::dolphin_schema::rss_sub_source::dsl::*;
    let connection = config::establish_connection();
    let query = rss_sub_source
        .filter(rep_lastest_refresh_time.lt(1485))
        .order(rep_lastest_refresh_time.asc())
        .limit(20);
    let query_result = query.load::<RssSubSource>(&connection).expect("load rss source failed");
    return query_result;
}

pub fn update_channel_reputation(new_reputation: i32,req_channel_id: i64) {
    use crate::model::diesel::dolphin::dolphin_schema::rss_sub_source::dsl::*;
    let connection = config::establish_connection();
    let predicate = crate::model::diesel::dolphin::dolphin_schema::rss_sub_source::id.eq(req_channel_id);
    diesel::update(rss_sub_source.filter(predicate))
        .set((reputation.eq(new_reputation)))
        .get_result::<RssSubSource>(&connection)
        .expect("unable to update app");
}
