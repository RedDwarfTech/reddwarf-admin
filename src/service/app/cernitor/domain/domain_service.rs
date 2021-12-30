use rocket::serde::json::Json;
use rust_wheel::common::query::pagination::{PaginateForQueryFragment};
use rust_wheel::common::util::model_convert::map_pagination_res;
use rust_wheel::common::util::time_util::get_current_millisecond;
use rust_wheel::config::db::config;
use rust_wheel::model::response::pagination_response::PaginationResponse;
use rust_wheel::model::user::login_user_info::LoginUserInfo;
use crate::diesel::prelude::*;
use crate::model::diesel::dolphin::dolphin_models::{Domain};
use crate::model::request::app::cernitor::domain::add_domain_request::AddDomainRequest;
use crate::model::request::app::cernitor::domain::domain_request::DomainRequest;

pub fn domain_query<T>(request: &Json<DomainRequest>) -> PaginationResponse<Vec<Domain>> {
    use crate::model::diesel::dolphin::dolphin_schema::domain::dsl::*;
    let connection = config::establish_connection();
    let query = domain.filter(id.gt(0))
        .paginate(request.pageNum)
        .per_page(request.pageSize);
    let query_result: QueryResult<(Vec<_>, i64, i64)> = query.load_and_count_pages_total::<Domain>(&connection);
    let page_result = map_pagination_res(query_result, request.pageNum, request.pageSize);
    return page_result;
}

pub fn add_domain(request: &Json<AddDomainRequest>, login_user_info: LoginUserInfo) {
    let connection = config::establish_connection();
    let timestamp: i64 = get_current_millisecond();

}



