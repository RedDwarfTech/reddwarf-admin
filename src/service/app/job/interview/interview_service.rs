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
use rust_wheel::model::user::login_user_info::LoginUserInfo;
use crate::model::diesel::dolphin::custom_dolphin_models::InterviewAdd;
use crate::model::request::app::job::interview::add_interview_request::AddInterviewRequest;
use crate::model::request::app::job::interview::edit_interview_request::EditInterviewRequest;

pub fn interview_query<T>(request: &Json<InterviewRequest>) -> PaginationResponse<Vec<Interview>> {
    use crate::model::diesel::dolphin::dolphin_schema::interview::dsl::*;
    let connection = config::establish_connection();
    let mut query = crate::model::diesel::dolphin::dolphin_schema::interview::table.into_boxed::<diesel::pg::Pg>();
    if let Some(query_company) = &request.company {
        query = query.filter(crate::model::diesel::dolphin::dolphin_schema::interview::company.eq(query_company));
    }
    let query = query
        .order(created_time.desc())
        .paginate(request.pageNum)
        .per_page(request.pageSize);

    let query_result: QueryResult<(Vec<_>, i64, i64)> = query.load_and_count_pages_total::<Interview>(&connection);
    let page_result = map_pagination_res(query_result, request.pageNum, request.pageSize);
    return page_result;
}

pub fn add_interview(request: &Json<AddInterviewRequest>,login_user_info: LoginUserInfo) {
    let connection = config::establish_connection();
    let current_time = get_current_millisecond();
    let app = InterviewAdd{
        city: request.city.to_string(),
        created_time: current_time,
        updated_time: current_time,
        company: request.company.to_string(),
        address: request.address.to_string(),
        status: 4,
        info_source: 1,
        salary_range: "9-15K".to_string(),
        apply_time: 0,
        apply_job: "Java开发工程师".to_string(),
        user_id: login_user_info.userId,
        job_link: "".to_string()
    };
    diesel::insert_into(crate::model::diesel::dolphin::dolphin_schema::interview::table)
        .values(&app)
        .on_conflict_do_nothing()
        .execute(&connection)
        .unwrap();
}

pub fn update_interview(request: &Json<EditInterviewRequest>) {
    use crate::model::diesel::dolphin::dolphin_schema::interview::dsl::*;
    let connection = config::establish_connection();
    let predicate = crate::model::diesel::dolphin::dolphin_schema::interview::id.eq(request.id);
    diesel::update(interview.filter(predicate))
        .set(
            (city.eq(&request.city),
             status.eq(&request.status),
             company.eq(&request.company),
             salary_range.eq(&request.salary_range),
             job_link.eq(&request.job_link),
             address.eq(&request.address))
        )
        .get_result::<Interview>(&connection)
        .expect("unable to update interview");
}