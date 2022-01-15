use rocket::serde::json::Json;
use rust_wheel::common::query::pagination::{PaginateForQueryFragment};
use rust_wheel::common::util::model_convert::map_pagination_res;
use rust_wheel::common::util::time_util::get_current_millisecond;
use rust_wheel::config::db::config;
use rust_wheel::model::response::pagination_response::PaginationResponse;
use crate::diesel::prelude::*;
use crate::model::diesel::dolphin::dolphin_models::App;
use crate::model::request::app::add_app_request::AddAppRequest;
use crate::model::request::app::app_request::AppRequest;

pub fn app_query<T>(request: &Json<AppRequest>) -> PaginationResponse<Vec<App>> {
    use crate::model::diesel::dolphin::dolphin_schema::apps::dsl::*;
    let connection = config::establish_connection();
    let query = apps.filter(id.gt(0))
        .paginate(request.pageNum)
        .per_page(request.pageSize);
    let query_result: QueryResult<(Vec<_>, i64, i64)> = query.load_and_count_pages_total::<App>(&connection);
    let page_result = map_pagination_res(query_result, request.pageNum, request.pageSize);
    return page_result;
}

pub fn app_create(request: &Json<AddAppRequest>) {
    use crate::model::diesel::dolphin::dolphin_schema::apps::dsl::*;
    let connection = config::establish_connection();
    let current_time = get_current_millisecond();
    let app = App{
        id: 0,
        app_name: request.appName.to_string(),
        remark: None,
        created_time: current_time,
        updated_time: Option::from(current_time),
        user_count: None,
        online_status: None,
        online_time: None,
        app_tag: None,
        app_id: 1,
        app_abbr: request.appAbbr.to_string()
    };
    diesel::insert_into(crate::model::diesel::dolphin::dolphin_schema::apps::table)
        .values(&app)
        .on_conflict_do_nothing()
        .execute(&connection)
        .unwrap();
}



