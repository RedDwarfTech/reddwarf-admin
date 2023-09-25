use diesel::ExpressionMethods;
use rocket::serde::json::Json;
use rust_wheel::common::query::pagination_fragment::PaginateForQueryFragment;
use rust_wheel::common::util::model_convert::map_pagination_res;
use rust_wheel::model::response::pagination_response::PaginationResponse;

use crate::common::db::database::get_conn;
use crate::diesel::prelude::*;
use crate::model::diesel::quark::custom_quark_models::AddSysDict;
use crate::model::diesel::quark::quark_models::SysDict;
use crate::model::request::sys::add_dict_request::AddDictRequest;
use crate::model::request::sys::sys_dict_request::SysDictRequest;

pub fn dict_query<T>() -> Vec<SysDict> {
    use crate::model::diesel::quark::quark_schema::sys_dict::dsl::*;
    let query = sys_dict.filter(id.gt(0))
        .limit(2000)
        .load::<SysDict>(&mut get_conn())
        .expect("query dict content failed");
    return query;
}

pub fn dict_page_query<T>(request: SysDictRequest) -> PaginationResponse<Vec<SysDict>> {
    let req = request.clone();
    use crate::model::diesel::quark::quark_schema::sys_dict as dict_table;
    let mut query = dict_table::table.into_boxed::<diesel::pg::Pg>();
    if let Some(dict_type_req) = request.dict_type {
        query = query.filter(dict_table::dict_type.eq(dict_type_req));
    }
    let query = query
        .paginate(req.pageNum.clone(),false)
        .per_page(req.pageSize.clone());
    let query_result: QueryResult<(Vec<_>, i64, i64)> = query.load_and_count_pages_total::<SysDict>(&mut get_conn());
    let page_result = map_pagination_res(query_result, req.pageNum, req.pageSize);
    return page_result;
}

pub fn dict_create(request: &Json<AddDictRequest>) {
    let app = AddSysDict{
        key: Option::from(request.key),
        dict_type: request.dict_type.to_string(),
        value: request.value.to_string(),
        show_value: request.show_value.to_string(),
        dict_type_name: request.dict_type_name.to_string()
    };
    diesel::insert_into(crate::model::diesel::quark::quark_schema::sys_dict::table)
        .values(&app)
        .on_conflict_do_nothing()
        .execute(&mut get_conn())
        .unwrap();
}