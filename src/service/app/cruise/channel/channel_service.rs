use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use num_traits::ToPrimitive;
use rocket::serde::json::Json;
use rust_wheel::common::query::pagination::PaginateForQueryFragment;
use rust_wheel::common::util::model_convert::map_pagination_res;
use rust_wheel::config::db::config;
use rust_wheel::model::response::pagination_response::PaginationResponse;
use rust_wheel::model::user::login_user_info::LoginUserInfo;

use crate::diesel::TextExpressionMethods;
use crate::model::diesel::dolphin::custom_dolphin_models::RssSubSourceUpdate;
use crate::model::diesel::dolphin::dolphin_models::RssSubSource;
use crate::model::request::app::cruise::channel::channel_request::ChannelRequest;
use crate::model::request::app::cruise::channel::update_channel_request::UpdateChannelRequest;

pub fn channel_query<T>(request: &Json<ChannelRequest>, _login_user_info: LoginUserInfo) -> PaginationResponse<Vec<RssSubSource>> {
    use crate::model::diesel::dolphin::dolphin_schema::rss_sub_source::dsl::*;
    use crate::model::diesel::dolphin::dolphin_schema::rss_sub_source as channel_table;
    let connection = config::establish_connection();
    let mut query = channel_table::table.into_boxed::<diesel::pg::Pg>();
    if let Some(edit_pick_req) = &request.editorPick {
        query = query.filter(channel_table::editor_pick.eq(edit_pick_req));
    }
    if let Some(minimal_rep_req) = &request.minimalReputation {
        query = query.filter(channel_table::reputation.ge(minimal_rep_req));
    }
    if let Some(_filter_tag) = &request.tag {
        let format_tags: serde_json::Value = serde_json::from_str(r#"{
            "street": "Article Circle Expressway 1"
        }"#).unwrap();
        query = query.filter(channel_table::tags.eq(format_tags));
    }
    if let Some(is_channel_tagged) = &request.isTag {
        if is_channel_tagged.to_i32().unwrap() == 0 {
            let format_tags: serde_json::Value = serde_json::from_str(r#"[]"#).unwrap();
            query = query.filter(channel_table::tags.eq(format_tags));
        }
    }
    if let Some(channel_sub_name) = &request.sub_name {
        query = query.filter(channel_table::sub_name.like(format!("{}{}{}","%",channel_sub_name.as_str(),"%")));
    }
    if let Some(current_sub_status) = &request.subStatus {
        query = query.filter(channel_table::sub_status.eq(current_sub_status));
    }

    let query = query
        .order(created_time.desc())
        .paginate(request.pageNum.unwrap_or(1),false)
        .per_page(request.pageSize.unwrap_or(10));
    let query_result: QueryResult<(Vec<_>, i64, i64)> = query.load_and_count_pages_total::<RssSubSource>(&connection);
    let page_result = map_pagination_res(query_result, request.pageNum.unwrap_or(1), request.pageSize.unwrap_or(10));
    return page_result;
}

pub fn update_channel(request: Json<UpdateChannelRequest>){
    if let Some(filter_tag) = &request.tags {
        let tag_json = serde_json::to_string(filter_tag);
        update_channel_tags(&request.channelId,tag_json.unwrap().to_string())
    }
    update_channel_impl(&request);  
}

pub fn update_channel_impl(request: &Json<UpdateChannelRequest>){
    use crate::model::diesel::dolphin::dolphin_schema::rss_sub_source::dsl::*;
    let connection = config::establish_connection();
    let predicate = crate::model::diesel::dolphin::dolphin_schema::rss_sub_source::id.eq(request.channelId);
    // https://diesel.rs/guides/all-about-updates.html
    // https://stackoverflow.com/questions/72249171/rust-diesel-conditionally-update-fields
    diesel::update(rss_sub_source.filter(predicate))
        .set(&RssSubSourceUpdate{
            part_output: request.partOutput.as_ref().unwrap().to_i32(),
            comment_rss: request.commentRss.as_ref().unwrap().to_i32(),
            sub_status: request.subStatus.as_ref().unwrap().to_i32()
        })
        .get_result::<RssSubSource>(&connection)
        .expect("unable to update channel");


    //diesel::update(rss_sub_source.filter(predicate))
    //    .set((comment_rss.eq(&request.commentRss),part_output.eq(&request.partOutput),sub_status.eq(&request.subStatus)))
    //    .get_result::<RssSubSource>(&connection)
    //    .expect("unable to update channel");
}

pub fn editor_pick_channel(req_channel_id: i64, editor_pick_status: i32){
    use crate::model::diesel::dolphin::dolphin_schema::rss_sub_source::dsl::*;
    let connection = config::establish_connection();
    let predicate = crate::model::diesel::dolphin::dolphin_schema::rss_sub_source::id.eq(req_channel_id);
    diesel::update(rss_sub_source.filter(predicate))
        .set(editor_pick.eq(editor_pick_status))
        .get_result::<RssSubSource>(&connection)
        .expect("unable to update channel");
}

pub fn update_channel_tags(req_channel_id: &i64, new_tags: String){
    use crate::model::diesel::dolphin::dolphin_schema::rss_sub_source::dsl::*;
    let connection = config::establish_connection();
    let predicate = crate::model::diesel::dolphin::dolphin_schema::rss_sub_source::id.eq(req_channel_id);
    let tag_db_str: serde_json::Value = serde_json::from_str(&*new_tags).unwrap_or_default();
    diesel::update(rss_sub_source.filter(predicate))
        .set(tags.eq(&tag_db_str))
        .get_result::<RssSubSource>(&connection)
        .expect("unable to update channel");
}

