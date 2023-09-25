use rocket::serde::json::Json;
use rust_wheel::common::query::pagination_fragment::PaginateForQueryFragment;
use rust_wheel::common::util::model_convert::map_pagination_res;
use rust_wheel::config::db::config;
use rust_wheel::model::response::pagination_response::PaginationResponse;

use crate::common::db::database::{ get_conn};
use crate::diesel::prelude::*;
use crate::model::diesel::dolphin::dolphin_models::{App, AppRepo};
use crate::model::request::app::add_app_request::AddAppRequest;
use crate::model::request::app::app_request::AppRequest;
use crate::model::request::app::gallery::repo_app_request::RepoAppRequest;

pub fn repo_app_query<T>(request: &Json<AppRequest>) -> PaginationResponse<Vec<AppRepo>> {
    use crate::model::diesel::dolphin::dolphin_schema::app_repo::dsl::*;
    
    let query = app_repo.filter(id.gt(0))
        .order(created_time.desc())
        .paginate(request.pageNum,false)
        .per_page(request.pageSize);
    let query_result: QueryResult<(Vec<_>, i64, i64)> = query.load_and_count_pages_total::<AppRepo>(&mut get_conn());
    let page_result = map_pagination_res(query_result, request.pageNum, request.pageSize);
    return page_result;
}

pub fn repo_app_create(_request: &Json<AddAppRequest>) {
   
}

pub fn repo_app_edit(request: &Json<RepoAppRequest>) {
    use crate::model::diesel::dolphin::dolphin_schema::apps::dsl::*;
    
    let predicate = crate::model::diesel::dolphin::dolphin_schema::apps::app_id.eq(request.appId.to_string());
    diesel::update(apps.filter(predicate))
        .set(remark.eq(&request.remark))
        .get_result::<App>(&mut get_conn())
        .expect("unable to update app");
}

pub fn repo_app_detail(query_app_id: i32) -> App {
    use crate::model::diesel::dolphin::dolphin_schema::apps::dsl::*;
    
    let app_result = apps.filter(id.eq(query_app_id))
        .first::<App>(&mut get_conn());
    return app_result.unwrap();
}