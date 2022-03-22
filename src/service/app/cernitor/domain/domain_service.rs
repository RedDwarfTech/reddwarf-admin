use rocket::serde::json::Json;
use rust_wheel::common::query::pagination::PaginateForQueryFragment;
use rust_wheel::common::util::model_convert::map_pagination_res;
use rust_wheel::common::util::time_util::get_current_millisecond;
use rust_wheel::config::db::config;
use rust_wheel::model::response::pagination_response::PaginationResponse;
use rust_wheel::model::user::login_user_info::LoginUserInfo;

use crate::diesel::prelude::*;
use crate::model::diesel::dolphin::dolphin_models::Domain;
use crate::model::request::app::cernitor::domain::add_domain_request::AddDomainRequest;
use crate::model::request::app::cernitor::domain::domain_request::DomainRequest;
use crate::model::request::app::cernitor::domain::edit_domain_request::EditDomainRequest;

pub fn domain_query<T>(request: &Json<DomainRequest>) -> PaginationResponse<Vec<Domain>> {
    use crate::model::diesel::dolphin::dolphin_schema::domain::dsl::*;
    let connection = config::establish_connection();
    let query = domain.filter(id.gt(0))
        .paginate(request.pageNum,false)
        .per_page(request.pageSize);
    let query_result: QueryResult<(Vec<_>, i64, i64)> = query.load_and_count_pages_total::<Domain>(&connection);
    let page_result = map_pagination_res(query_result, request.pageNum, request.pageSize);
    return page_result;
}

pub fn add_domain(request: &Json<AddDomainRequest>, login_user_info: LoginUserInfo) {
    let connection = config::establish_connection();
    let timestamp: i64 = get_current_millisecond();
    let new_domain = crate::model::diesel::dolphin::custom_dolphin_models::Domain {
        domain_name: request.domainName.to_string(),
        domain_url: request.domainUrl.to_string(),
        created_time: timestamp,
        updated_time: timestamp,
        cron: Some("* */1 * * * *".parse().unwrap()),
        next_trigger_time: None,
        monitor_status: 1,
        user_id: Option::from(login_user_info.userId),
        expire_date: None,
        days_before_trigger: 7,
        notify_trigger_date: None,
        expire_date_ms: None,
    };
    diesel::insert_into(crate::model::diesel::dolphin::dolphin_schema::domain::table)
        .values(&new_domain)
        .on_conflict_do_nothing()
        .execute(&connection)
        .unwrap();
}

pub fn edit_domain(request: &Json<EditDomainRequest>, login_user_info: LoginUserInfo) {
    let connection = config::establish_connection();
    use crate::model::diesel::dolphin::dolphin_schema::domain::dsl::*;
    let predicate = crate::model::diesel::dolphin::dolphin_schema::domain::id.eq(request.id)
        .and(crate::model::diesel::dolphin::dolphin_schema::domain::user_id.eq(login_user_info.userId));
    diesel::update(domain.filter(predicate))
        .set((domain_url.eq(&request.domainUrl),domain_name.eq(&request.domainName)))
        .get_result::<Domain>(&connection)
        .expect("unable to update new password");
}

