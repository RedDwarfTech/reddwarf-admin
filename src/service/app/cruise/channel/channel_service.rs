use diesel::{BoxableExpression, ExpressionMethods, QueryDsl, QueryResult};
use diesel::sql_types::Bool;
use rocket::serde::json::Json;
use rust_wheel::common::query::pagination::PaginateForQueryFragment;
use rust_wheel::common::util::model_convert::map_pagination_res;
use rust_wheel::config::db::config;
use rust_wheel::model::response::pagination_response::PaginationResponse;

use crate::model::diesel::dolphin::dolphin_models::RssSubSource;
use crate::model::request::app::cruise::channel::channel_request::ChannelRequest;

type DB = diesel::pg::Pg;

pub fn channel_query<T>(request: &Json<ChannelRequest>) -> PaginationResponse<Vec<RssSubSource>> {
    use crate::model::diesel::dolphin::dolphin_schema::rss_sub_source::dsl::*;
    let connection = config::establish_connection();
    let query = rss_sub_source
        .filter(find_channel(&request.0))
        .order(created_time.desc())
        .paginate(1)
        .per_page(10);
    let query_result: QueryResult<(Vec<_>, i64, i64)> = query.load_and_count_pages_total::<RssSubSource>(&connection);
    let page_result = map_pagination_res(
        query_result,
        1,
        10);
    return page_result;
}


fn find_channel(request: &ChannelRequest) -> Box<dyn BoxableExpression<crate::model::diesel::dolphin::dolphin_schema::rss_sub_source::table, DB, SqlType=Bool> + '_> {
    use crate::model::diesel::dolphin::dolphin_schema::rss_sub_source::dsl::*;
    match request {
        ChannelRequest { editorPick, .. } if editorPick.unwrap_or(0) == 1 => Box::new(editor_pick.eq(editorPick)),
        ChannelRequest { minimalReputation, ..} if minimalReputation.unwrap_or(0) > 0 => Box::new(reputation.gt(minimalReputation)),
        ChannelRequest { excludeEditorPickChannel, ..} if excludeEditorPickChannel.unwrap_or(0) == 1 => Box::new(editor_pick.ne(excludeEditorPickChannel)),
        _ => Box::new(editor_pick.eq(0))
    }
}

