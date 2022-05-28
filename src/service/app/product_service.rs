use rocket::serde::json::Json;
use rust_wheel::common::query::pagination::PaginateForQueryFragment;
use rust_wheel::common::util::collection_util::take;
use rust_wheel::common::util::model_convert::map_pagination_res;
use rust_wheel::common::util::time_util::get_current_millisecond;
use rust_wheel::config::db::config;
use rust_wheel::model::response::pagination_response::PaginationResponse;

use crate::diesel::prelude::*;
use crate::model::diesel::dolphin::custom_dolphin_models::{AppAdd, ProductAdd};
use crate::model::diesel::dolphin::dolphin_models::{App, Product};
use crate::model::request::app::add_app_request::AddAppRequest;
use crate::model::request::app::add_product_request::AddProductRequest;
use crate::model::request::app::edit_app_request::EditAppRequest;
use crate::model::request::app::product_request::ProductRequest;

pub fn product_query<T>(request: &Json<ProductRequest>) -> PaginationResponse<Vec<Product>> {
    use crate::model::diesel::dolphin::dolphin_schema::products::dsl::*;
    let connection = config::establish_connection();
    let query = products.filter(id.gt(0))
        .order(created_time.desc())
        .paginate(request.pageNum,false)
        .per_page(request.pageSize);
    let query_result: QueryResult<(Vec<_>, i64, i64)> = query.load_and_count_pages_total::<Product>(&connection);
    let page_result = map_pagination_res(query_result, request.pageNum, request.pageSize);
    return page_result;
}

pub fn product_create(request: &Json<AddProductRequest>) {
    use crate::model::diesel::dolphin::dolphin_schema::products::dsl::*;
    let connection = config::establish_connection();
    let apps_record = products.order(product_id.desc())
        .limit(1)
        .load::<Product>(&connection)
        .expect("query app failed");
    let app_db = take(apps_record,0).unwrap();

    let current_time = get_current_millisecond();
    let app = ProductAdd{
        product_name: request.productName.to_string(),
        remark: request.remark.to_string(),
        created_time: current_time,
        updated_time: current_time,
        user_count: 0,
        online_status: 1,
        online_time: None,
        product_tag: None,
        product_id: app_db.product_id,
        product_abbr: request.productAbbr.to_string()
    };
    diesel::insert_into(crate::model::diesel::dolphin::dolphin_schema::products::table)
        .values(&app)
        .on_conflict_do_nothing()
        .execute(&connection)
        .unwrap();
}

pub fn product_edit(request: &Json<EditAppRequest>) {
    use crate::model::diesel::dolphin::dolphin_schema::apps::dsl::*;
    let connection = config::establish_connection();
    let predicate = crate::model::diesel::dolphin::dolphin_schema::apps::app_id.eq(request.appId.to_string());
    diesel::update(apps.filter(predicate))
        .set(remark.eq(&request.remark))
        .get_result::<App>(&connection)
        .expect("unable to update app");
}

pub fn product_detail(query_app_id: i32) -> App {
    use crate::model::diesel::dolphin::dolphin_schema::apps::dsl::*;
    let connection = config::establish_connection();
    let app_result = apps.filter(id.eq(query_app_id))
        .first::<App>(&connection);
    return app_result.unwrap();
}