use num_traits::ToPrimitive;
use rocket::serde::json::Json;
use rust_wheel::common::query::pagination::{PaginateForQueryFragment};
use rust_wheel::common::util::model_convert::map_pagination_res;
use rust_wheel::common::util::time_util::get_current_millisecond;
use rust_wheel::config::db::config;
use rust_wheel::model::response::pagination_response::PaginationResponse;
use crate::diesel::prelude::*;
use crate::model::diesel::dict::dict_models::{Dict, UserDict};
use crate::model::diesel::dolphin::dolphin_models::App;
use crate::model::request::app::app_request::AppRequest;
use crate::model::request::app::dict::translate::translate_request::TranslateRequest;
use crate::model::request::app::dict::word::glossary_add_request::GlossaryAddRequest;

pub fn glossary_query(request: &Json<TranslateRequest>) -> Vec<UserDict> {
    use crate::model::diesel::dict::dict_schema::user_dict::dsl::*;
    let connection = config::establish_dict_connection();
    let user_dicts:Vec<UserDict> = user_dict
        .limit(100)
        .load::<UserDict>(&connection)
        .expect("query user dict failed");
    let words: Vec<String> = user_dicts
        .iter()
        .map(|item| item.word.to_string())
        .collect();
    return user_dicts;
}

pub fn glossary_add(request: &Json<GlossaryAddRequest>){
    use crate::model::diesel::dict::dict_schema::user_dict::dsl::*;
    let connection = config::establish_dict_connection();
    let timestamp: i64 = get_current_millisecond();
    let new_glossary = crate::model::diesel::dict::dict_models::UserDict {
        id: 0,
        created_time: timestamp,
        updated_time: timestamp,
        status: 0,
        user_id: request.userId,
        word_id: 0,
        word: request.word.to_string()
    };
    diesel::insert_into(crate::model::diesel::dict::dict_schema::user_dict::table)
        .values(&new_glossary)
        .on_conflict_do_nothing()
        .execute(&connection)
        .unwrap();
}



