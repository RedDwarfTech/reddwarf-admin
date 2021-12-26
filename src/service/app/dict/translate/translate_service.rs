use rocket::serde::json::Json;
use rust_wheel::common::query::pagination::{PaginateForQueryFragment};
use rust_wheel::common::util::model_convert::map_pagination_res;
use rust_wheel::config::db::config;
use rust_wheel::model::response::pagination_response::PaginationResponse;
use crate::diesel::prelude::*;
use crate::model::diesel::dolphin::dolphin_models::App;
use crate::model::request::app::app_request::AppRequest;

pub fn translate_query<T>(request: &Json<AppRequest>) -> PaginationResponse<Vec<App>> {
    use crate::model::diesel::dolphin::dolphin_schema::apps::dsl::*;
    let connection = config::establish_connection();
    let query = apps.filter(id.gt(0))
        .paginate(request.pageNum)
        .per_page(request.pageSize);
    let query_result: QueryResult<(Vec<_>, i64, i64)> = query.load_and_count_pages_total::<App>(&connection);
    let page_result = map_pagination_res(query_result, request.pageNum, request.pageSize);
    return page_result;
}





