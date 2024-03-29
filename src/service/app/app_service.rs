use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rocket::serde::json::Json;
use rust_wheel::common::query::pagination_fragment::PaginateForQueryFragment;
use rust_wheel::common::util::model_convert::map_pagination_from_list;
use rust_wheel::common::util::time_util::get_current_millisecond;
use rust_wheel::model::response::pagination_response::PaginationResponse;
use crate::common::db::database::get_conn;
use crate::diesel::prelude::*;
use crate::model::diesel::dolphin::custom_dolphin_models::AppAdd;
use crate::model::diesel::dolphin::dolphin_models::{App, Product};
use crate::model::request::app::add_app_request::AddAppRequest;
use crate::model::request::app::app_request::AppRequest;
use crate::model::request::app::edit_app_request::EditAppRequest;
use crate::model::response::app::app_response::AppResponse;

pub fn app_query<T>(request: &Json<AppRequest>) -> PaginationResponse<Vec<AppResponse>> {
    use crate::model::diesel::dolphin::dolphin_schema::apps::dsl::*;
    let query = apps
        .filter(id.gt(0))
        .order(created_time.desc())
        .paginate(request.pageNum, false)
        .per_page(request.pageSize);
    let query_result: QueryResult<(Vec<_>, i64, i64)> =
        query.load_and_count_pages_total::<App>(&mut get_conn());
    let app_response = append_product_name(&query_result.as_ref().unwrap().0);
    let total = query_result.as_ref().unwrap().2;
    let page_result =
        map_pagination_from_list(app_response, request.pageNum, request.pageSize, total);
    return page_result;
}

pub fn append_product_name(apps: &Vec<App>) -> Vec<AppResponse> {
    use crate::model::diesel::dolphin::dolphin_schema::products::dsl::*;
    let product_ids: Vec<i32> = apps.iter().map(|item| item.product_id).collect();
    let products_result = products
        .filter(product_id.eq_any(product_ids))
        .load::<Product>(&mut get_conn())
        .expect("query product source failed");
    let mut app_res = Vec::new();
    for app_temp in apps {
        let fetched_product_name: String = products_result
            .iter()
            .filter(|prod| prod.product_id == app_temp.product_id)
            .map(|channel| channel.product_name.to_string())
            .collect::<String>();
        let mut app_response = AppResponse::from(app_temp);
        app_response.product_name = fetched_product_name;
        app_res.push(app_response);
    }
    return app_res;
}

pub fn app_create(request: &Json<AddAppRequest>) {
    let current_time = get_current_millisecond();
    // https://stackoverflow.com/questions/65478444/how-to-generate-a-random-string-of-alphanumeric-chars
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .map(char::from)
        .take(10)
        .collect();
    let app = AppAdd {
        app_name: request.appName.to_string(),
        remark: request.remark.to_string(),
        created_time: current_time,
        updated_time: current_time,
        user_count: 0,
        online_status: 0,
        online_time: None,
        app_tag: None,
        app_id: rand_string,
        app_abbr: request.appAbbr.to_string(),
        product_id: request.productId,
    };
    diesel::insert_into(crate::model::diesel::dolphin::dolphin_schema::apps::table)
        .values(&app)
        .on_conflict_do_nothing()
        .execute(&mut get_conn())
        .unwrap();
}

pub fn app_edit(request: &Json<EditAppRequest>) {
    use crate::model::diesel::dolphin::dolphin_schema::apps::dsl::*;
    let predicate = crate::model::diesel::dolphin::dolphin_schema::apps::id.eq(request.id);
    diesel::update(apps.filter(predicate))
        .set(remark.eq(&request.remark))
        .get_result::<App>(&mut get_conn())
        .expect("unable to update app");
}

pub fn app_detail(query_app_id: i32) -> App {
    use crate::model::diesel::dolphin::dolphin_schema::apps::dsl::*;
    let app_result = apps.filter(id.eq(query_app_id)).first::<App>(&mut get_conn());
    return app_result.unwrap();
}
