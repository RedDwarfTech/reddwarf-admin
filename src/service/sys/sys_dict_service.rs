use diesel::ExpressionMethods;
use rocket::serde::json::Json;
use rust_wheel::common::query::pagination::PaginateForQueryFragment;
use rust_wheel::common::util::model_convert::map_pagination_res;
use rust_wheel::config::db::config;
use rust_wheel::model::response::pagination_response::PaginationResponse;

use crate::diesel::prelude::*;
use crate::model::diesel::quark::quark_models::SysDict;
use crate::model::request::sys::sys_dict_request::SysDictRequest;

pub fn dict_query<T>() -> Vec<SysDict> {
    use crate::model::diesel::quark::quark_schema::sys_dict::dsl::*;
    let connection = config::establish_quark_connection();
    let query = sys_dict.filter(id.gt(0))
        .limit(2000)
        .load::<SysDict>(&connection)
        .expect("query dict content failed");
    return query;
}

pub fn dict_page_query<T>(request: Json<SysDictRequest>) -> PaginationResponse<Vec<SysDict>> {
    use crate::model::diesel::quark::quark_schema::sys_dict as dict_table;
    let connection = config::establish_quark_connection();
    let query = dict_table::table.into_boxed::<diesel::pg::Pg>();
    let query = query
        .paginate(request.pageNum,false)
        .per_page(request.pageSize);
    let query_result: QueryResult<(Vec<_>, i64, i64)> = query.load_and_count_pages_total::<SysDict>(&connection);
    let page_result = map_pagination_res(query_result, request.pageNum, request.pageSize);
    return page_result;
}
