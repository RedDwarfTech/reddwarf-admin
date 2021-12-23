use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use rocket::serde::json::Json;
use rust_wheel::common::query::pagination::PaginateForQueryFragment;
use rust_wheel::common::util::collection_util::take;
use rust_wheel::common::util::model_convert::map_pagination_res;
use rust_wheel::model::response::pagination_response::PaginationResponse;
use rust_wheel::config::db::config;
use crate::model::diesel::dolphin::dolphin_models::{ RssSubSource};
use crate::model::request::app::cruise::channel::channel_request::ChannelRequest;

pub fn channel_query<T>(request: &Json<ChannelRequest>) -> PaginationResponse<Vec<RssSubSource>> {
    use crate::model::diesel::dolphin::dolphin_schema::rss_sub_source::dsl::*;
    let connection = config::establish_connection();
    let query = rss_sub_source
        .filter(id.gt(0))
        .order(created_time.desc())
        .paginate(request.pageNum)
        .per_page(request.pageSize);
    let query_result: QueryResult<(Vec<_>, i64, i64)> = query.load_and_count_pages_total::<RssSubSource>(&connection);
    let page_result = map_pagination_res(query_result, request.pageNum, request.pageSize);
    return page_result;
}

