use rocket::serde::json::Json;
use rust_wheel::common::query::pagination::PaginateForQueryFragment;
use rust_wheel::common::util::collection_util::take;
use rust_wheel::common::util::model_convert::map_pagination_res;
use rust_wheel::common::util::time_util::get_current_millisecond;
use rust_wheel::config::db::config;
use rust_wheel::model::response::pagination_response::PaginationResponse;

use crate::diesel::prelude::*;
use crate::model::diesel::dolphin::dolphin_models::{App, AppRepo};
use crate::model::request::app::add_app_request::AddAppRequest;
use crate::model::request::app::app_request::AppRequest;
use crate::model::request::app::gallery::repo_app_request::RepoAppRequest;

pub fn repo_app_query<T>(request: &Json<AppRequest>) -> PaginationResponse<Vec<AppRepo>> {
    use crate::model::diesel::dolphin::dolphin_schema::app_repo::dsl::*;
    let connection = config::establish_connection();
    let query = app_repo.filter(id.gt(0))
        .order(created_time.desc())
        .paginate(request.pageNum,false)
        .per_page(request.pageSize);
    let query_result: QueryResult<(Vec<_>, i64, i64)> = query.load_and_count_pages_total::<AppRepo>(&connection);
    let page_result = map_pagination_res(query_result, request.pageNum, request.pageSize);
    return page_result;
}

pub fn repo_app_create(request: &Json<AddAppRequest>) {
    use crate::model::diesel::dolphin::dolphin_schema::apps::dsl::*;
    let connection = config::establish_connection();
    let apps_record = apps.order(app_id.desc())
        .limit(1)
        .load::<App>(&connection)
        .expect("query app  failed");
    let app_db = take(apps_record,0).unwrap();

    let current_time = get_current_millisecond();
   
}

pub fn repo_app_edit(request: &Json<RepoAppRequest>) {
    use crate::model::diesel::dolphin::dolphin_schema::apps::dsl::*;
    let connection = config::establish_connection();
    let predicate = crate::model::diesel::dolphin::dolphin_schema::apps::app_id.eq(request.appId.to_string());
    diesel::update(apps.filter(predicate))
        .set(remark.eq(&request.remark))
        .get_result::<App>(&connection)
        .expect("unable to update app");
}

pub fn repo_app_detail(query_app_id: i32) -> App {
    use crate::model::diesel::dolphin::dolphin_schema::apps::dsl::*;
    let connection = config::establish_connection();
    let app_result = apps.filter(id.eq(query_app_id))
        .first::<App>(&connection);
    return app_result.unwrap();
}