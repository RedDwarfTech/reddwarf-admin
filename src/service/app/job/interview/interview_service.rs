use diesel::QueryResult;
use rocket::serde::json::Json;
use rust_wheel::common::query::pagination::PaginateForQueryFragment;
use rust_wheel::common::util::model_convert::map_pagination_res;
use rust_wheel::common::util::time_util::get_current_millisecond;
use rust_wheel::model::response::pagination_response::PaginationResponse;
use crate::model::diesel::dolphin::dolphin_models::Interview;
use crate::model::request::app::job::interview::interview_request::InterviewRequest;
use crate::diesel::prelude::*;
use rust_wheel::config::db::config;
use crate::model::diesel::dolphin::custom_dolphin_models::InterviewAdd;
use crate::model::request::app::job::interview::add_interview_request::AddInterviewRequest;

pub fn interview_query<T>(request: &Json<InterviewRequest>) -> PaginationResponse<Vec<Interview>> {
    use crate::model::diesel::dolphin::dolphin_schema::interview::dsl::*;
    let connection = config::establish_connection();
    let query = interview.filter(id.gt(0))
        .order(created_time.desc())
        .paginate(request.pageNum)
        .per_page(request.pageSize);
    let query_result: QueryResult<(Vec<_>, i64, i64)> = query.load_and_count_pages_total::<Interview>(&connection);
    let page_result = map_pagination_res(query_result, request.pageNum, request.pageSize);
    return page_result;
}

pub fn add_interview(request: &Json<AddInterviewRequest>) {
    use crate::model::diesel::dolphin::dolphin_schema::interview::dsl::*;
    let connection = config::establish_connection();
    let current_time = get_current_millisecond();
    let app = InterviewAdd{
        city: request.city.to_string(),
        created_time: current_time,
        updated_time: current_time,
        company: request.company.to_string(),
        address: request.address.to_string()
    };
    diesel::insert_into(crate::model::diesel::dolphin::dolphin_schema::interview::table)
        .values(&app)
        .on_conflict_do_nothing()
        .execute(&connection)
        .unwrap();
}