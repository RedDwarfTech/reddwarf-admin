use rocket::serde::json::Json;
use rust_wheel::common::query::pagination::{PaginateForQueryFragment};
use rust_wheel::common::util::model_convert::map_pagination_res;
use rust_wheel::config::db::config;
use rust_wheel::model::response::pagination_response::PaginationResponse;
use crate::diesel::prelude::*;
use crate::model::diesel::dict::dict_models::Dict;
use crate::model::diesel::dolphin::dolphin_models::App;
use crate::model::request::app::app_request::AppRequest;
use crate::model::request::app::dict::translate::translate_request::TranslateRequest;

pub fn translate_query(request: &Json<TranslateRequest>) -> Vec<Dict> {
    use crate::model::diesel::dict::dict_schema::dict::dsl::*;
    let connection = config::establish_connection();
    let query = dict.filter(word.eq(&request.word))
        .limit(1)
        .load::<Dict>(&connection)
        .expect("query dict id failed");
    return query;
}





