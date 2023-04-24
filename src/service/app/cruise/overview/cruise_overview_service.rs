use diesel::{BoolExpressionMethods, QueryDsl, RunQueryDsl};
use diesel::dsl::any;
use rocket::serde::json::Json;
use rust_wheel::common::util::time_util::get_current_millisecond;
use rust_wheel::config::db::config;

use crate::diesel::ExpressionMethods;
use crate::model::diesel::dolphin::custom_dolphin_models::TrendAdd;
use crate::model::diesel::dolphin::dolphin_models::{Article, Trend};
use crate::model::diesel::dolphin::dolphin_schema::article::sub_source_id;
use crate::model::diesel::dolphin::dolphin_schema::article_content::article_id;
use crate::model::diesel::dolphin::dolphin_schema::rss_sub_source::{article_count, article_count_latest_refresh_time};
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

pub fn delete_legacy_article() {
    use crate::model::diesel::dolphin::dolphin_schema::article as article_table;
    let connection = config::establish_connection();
    let predicate = crate::model::diesel::dolphin::dolphin_schema::article::created_time.lt(get_current_millisecond()-1000*60*60*24*90);
    let articles = article_table::table
        .filter(predicate)
        .order(article_table::created_time.asc())
        .limit(30)
        .load::<Article>(&connection)
        .expect("query old articles failed");
    if articles.is_empty() {
        return;
    }
    let article_ids = articles.iter()
        .map(|item| item.id)
        .collect();
    let transaction_result = connection.build_transaction()
        .repeatable_read()
        .run::<_, diesel::result::Error, _>(||{
            delete_article(&article_ids);
            delete_article_detail(&article_ids);
            Ok(())
        });
    match transaction_result {
        Ok(_v) => {

        },
        Err(e) =>{
            error!("error:{}",e.to_string());
        },
    }
}

pub fn delete_low_quality_channel(filter_channel_id: i64) {
    use crate::model::diesel::dolphin::dolphin_schema::article as article_table;
    let connection = config::establish_connection();
    let predicate = sub_source_id.eq(filter_channel_id);
    let articles = article_table::table
        .filter(predicate)
        .limit(50)
        .load::<Article>(&connection)
        .expect("query articles failed");
    if articles.is_empty() {
        update_channel_article_count(filter_channel_id, 0);
        return;
    }
    let article_ids = articles.iter()
        .map(|item| item.id)
        .collect();
    let transaction_result = connection.build_transaction()
        .repeatable_read()
        .run::<_, diesel::result::Error, _>(||{
            delete_article(&article_ids);
            delete_article_detail(&article_ids);
            Ok(())
        });
    match transaction_result {
        Ok(_v) => {

        },
        Err(e) =>{
            error!("error:{}",e.to_string());
        },
    }
}

pub fn delete_article(ids: &Vec<i64>){
    use crate::model::diesel::dolphin::dolphin_schema::article as article_table;
    let connection = config::establish_connection();
    let predicate = article_table::dsl::id.eq(any(ids));
    diesel::delete(article_table::table.filter(predicate))
        .execute(&connection).expect("delete article failed");
}

pub fn delete_article_detail(ids: &Vec<i64>){
    use crate::model::diesel::dolphin::dolphin_schema::article_content as article_detail_table;
    let connection = config::establish_connection();
    let predicate = article_id.eq(any(ids));
    diesel::delete(article_detail_table::table.filter(predicate))
        .execute(&connection).expect("delete article detail failed");
}

pub fn update_channel_article_count(filter_channel_id: i64, new_article_count: i64){
    use crate::model::diesel::dolphin::dolphin_schema::rss_sub_source as channel_table;
    let connection = config::establish_connection();
    let predicate = channel_table::dsl::id.eq(filter_channel_id);
    let current_time = get_current_millisecond();
    diesel::update(channel_table::table.filter(predicate))
        .set((article_count.eq(new_article_count),article_count_latest_refresh_time.eq(current_time)))
        .execute(&connection)
        .expect("unable to update channel article count");
}

