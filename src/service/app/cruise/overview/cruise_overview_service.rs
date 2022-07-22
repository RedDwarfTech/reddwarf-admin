use diesel::{BoolExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::serde::json::Json;
use rust_wheel::config::db::config;

use crate::diesel::ExpressionMethods;
use crate::model::diesel::dolphin::custom_dolphin_models::TrendAdd;
use crate::model::diesel::dolphin::dolphin_models::Trend;
use crate::model::request::app::cruise::overview::cruise_overview_request::CruiseOverviewRequest;

pub fn cruise_trend_query(request: &Json<CruiseOverviewRequest>) -> Vec<Trend> {
    use crate::model::diesel::dolphin::dolphin_schema::trend as trend_table;
    let connection = config::establish_connection();
    let predicate = trend_table::dsl::statistic_time.ge(request.startTime).and(
        trend_table::dsl::statistic_time.le(request.endTime)
    );
    let trend_records = trend_table::table
    .filter(predicate)
    .load::<Trend>(&connection)
    .expect("query trends records failed");
    return trend_records;
}

pub fn update_days_article_count(new_trend: &TrendAdd) {
    use crate::model::diesel::dolphin::dolphin_schema::trend::dsl::*;
    let connection = config::establish_connection();
    diesel::insert_into(trend)
        .values(new_trend)
        .on_conflict((statistic_time,app_id,trend_item))
        .do_update()
        .set(incre_num.eq(new_trend.incre_num))
        .execute(&connection)
        .expect("unable to update trend article count");
}

pub fn delete_low_quality_channel(filter_channel_id: i64) {
    let connection = config::establish_connection();
    // https://stackoverflow.com/questions/5170546/how-do-i-delete-a-fixed-number-of-rows-with-sorting-in-postgresql
    diesel::sql_query(format!("delete FROM article WHERE ctid in(select ctid from article where channel_id ={} limit 50)",filter_channel_id))
        .execute(&connection)
        .expect("An error has occured");
}
