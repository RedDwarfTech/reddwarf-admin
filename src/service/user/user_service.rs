use rocket::serde::json::Json;
use rust_wheel::common::query::pagination::{PaginateForQueryFragment};
use rust_wheel::common::util::model_convert::map_pagination_res;
use rust_wheel::config::db::config;
use rust_wheel::model::response::pagination_response::PaginationResponse;
use crate::diesel::prelude::*;
use crate::model::request::user::user_request::UserRequest;
use crate::model::diesel::dolphin::dolphin_models::User;

pub fn user_query<T>(request: &Json<UserRequest>) -> PaginationResponse<Vec<User>> {
    use crate::model::diesel::dolphin::dolphin_schema::users::dsl::*;
    let connection = config::establish_connection();
    let query = users.filter(id.gt(0))
        .paginate(request.pageNum)
        .per_page(request.pageSize);
    let query_result: QueryResult<(Vec<_>, i64, i64)> = query.load_and_count_pages_total::<User>(&connection);
    let page_result = map_pagination_res(query_result, request.pageNum, request.pageSize);
    return page_result;
}

