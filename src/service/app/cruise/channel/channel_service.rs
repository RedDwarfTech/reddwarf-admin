use diesel::{BoxableExpression, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use diesel::sql_types::Bool;
use rocket::serde::json::Json;
use rust_wheel::common::query::pagination::PaginateForQueryFragment;
use rust_wheel::common::util::model_convert::map_pagination_res;
use rust_wheel::config::db::config;
use rust_wheel::model::response::pagination_response::PaginationResponse;
use rust_wheel::model::user::login_user_info::LoginUserInfo;

use crate::model::diesel::dolphin::dolphin_models::RssSubSource;
use crate::model::request::app::cruise::channel::channel_request::ChannelRequest;

type DB = diesel::pg::Pg;

pub fn channel_query<T>(request: &Json<ChannelRequest>, login_user_info: LoginUserInfo) -> PaginationResponse<Vec<RssSubSource>> {
    use crate::model::diesel::dolphin::dolphin_schema::rss_sub_source::dsl::*;
    use crate::model::diesel::dolphin::dolphin_schema::rss_sub_source as channel_table;
    let connection = config::establish_connection();
    let mut query = channel_table::table.into_boxed::<diesel::pg::Pg>();
    if let Some(edit_pick_req) = &request.editorPick {
        query = query.filter(channel_table::editor_pick.eq(edit_pick_req));
    }
    let query = query
        .order(created_time.desc())
        .paginate(request.pageNum.unwrap_or(1),false)
        .per_page(request.pageSize.unwrap_or(10));
    let query_result: QueryResult<(Vec<_>, i64, i64)> = query.load_and_count_pages_total::<RssSubSource>(&connection);
    let page_result = map_pagination_res(
        query_result,
        1,
        10);
    return page_result;
}

pub fn editor_pick_channel(req_channel_id: i64){
    use crate::model::diesel::dolphin::dolphin_schema::rss_sub_source::dsl::*;
    let connection = config::establish_connection();
    let predicate = crate::model::diesel::dolphin::dolphin_schema::rss_sub_source::id.eq(req_channel_id);
    diesel::update(rss_sub_source.filter(predicate))
        .set((editor_pick.eq(1)))
        .get_result::<RssSubSource>(&connection)
        .expect("unable to update channel");
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

