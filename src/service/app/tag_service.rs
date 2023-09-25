use rocket::serde::json::Json;
use rust_wheel::common::query::pagination_fragment::PaginateForQueryFragment;
use rust_wheel::common::util::collection_util::take;
use rust_wheel::common::util::model_convert::map_pagination_res;
use rust_wheel::common::util::time_util::get_current_millisecond;
use rust_wheel::config::db::config;
use rust_wheel::model::response::pagination_response::PaginationResponse;

use crate::common::db::database::get_conn;
use crate::diesel::prelude::*;
use crate::model::diesel::dolphin::custom_dolphin_models::{TagAdd};
use crate::model::diesel::dolphin::dolphin_models::{Tag};
use crate::model::request::app::add_app_request::AddAppRequest;
use crate::model::request::app::app_request::AppRequest;
use crate::model::request::app::edit_app_request::EditAppRequest;
use crate::model::request::app::tag_list_request::TagListRequest;

pub fn tag_query<T>(request: &Json<AppRequest>) -> PaginationResponse<Vec<Tag>> {
    use crate::model::diesel::dolphin::dolphin_schema::tags::dsl::*;
    let connection = config::establish_connection();
    let query = tags.filter(id.gt(0))
        .order(created_time.desc())
        .paginate(request.pageNum,false)
        .per_page(request.pageSize);
    let query_result: QueryResult<(Vec<_>, i64, i64)> = query.load_and_count_pages_total::<Tag>(&mut get_conn());
    let page_result = map_pagination_res(query_result, request.pageNum, request.pageSize);
    return page_result;
}

pub fn tag_query_list(_request: &Json<TagListRequest>) -> Vec<Tag> {
    use crate::model::diesel::dolphin::dolphin_schema::tags::dsl::*;
    let connection = config::establish_connection();
    let tags_record = tags.order(app_id.desc())
        .load::<Tag>(&mut get_conn())
        .expect("query tags failed");
    return tags_record;
}

pub fn tag_create(_request: &Json<AddAppRequest>) {
    use crate::model::diesel::dolphin::dolphin_schema::tags::dsl::*;
    let connection = config::establish_connection();
    let apps_record = tags.order(app_id.desc())
        .limit(1)
        .load::<Tag>(&mut get_conn())
        .expect("query app  failed");
    let app_db = take(apps_record,0).unwrap();

    let current_time = get_current_millisecond();
    let app = TagAdd{
        tag_name: "".to_string(),
        remark: None,
        group: 0,
        tag_type: "".to_string(),
        created_time: current_time,
        updated_time: current_time,
        app_id: app_db.app_id,
        code: "".to_string()
    };
    diesel::insert_into(crate::model::diesel::dolphin::dolphin_schema::tags::table)
        .values(&app)
        .on_conflict_do_nothing()
        .execute(&mut get_conn())
        .unwrap();
}

pub fn tag_edit(_request: &Json<EditAppRequest>) {


}

pub fn tag_detail(query_app_id: i32) -> Tag {
    use crate::model::diesel::dolphin::dolphin_schema::tags::dsl::*;
    let connection = config::establish_connection();
    let app_result = tags.filter(id.eq(query_app_id))
        .first::<Tag>(&mut get_conn());
    return app_result.unwrap();
}