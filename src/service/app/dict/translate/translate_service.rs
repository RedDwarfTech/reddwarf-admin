use rocket::serde::json::Json;
use crate::common::db::database::get_conn;
use crate::diesel::prelude::*;
use crate::model::diesel::dict::dict_models::Dict;
use crate::model::request::app::dict::translate::translate_request::TranslateRequest;

pub fn translate_query(request: &Json<TranslateRequest>) -> Vec<Dict> {
    use crate::model::diesel::dict::dict_schema::dict::dsl::*;
    let query = dict.filter(word.eq(&request.word))
        .limit(1)
        .load::<Dict>(&mut get_conn())
        .expect("query dict id failed");
    return query;
}





