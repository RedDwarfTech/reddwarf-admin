use rocket::serde::json::Json;
use rust_wheel::common::util::time_util::get_current_millisecond;
use rust_wheel::config::db::config;

use crate::diesel::prelude::*;
use crate::model::diesel::dict::custom_dict_models::CustomUserDict;
use crate::model::diesel::dict::dict_models::{Dict, UserDict};
use crate::model::request::app::dict::word::glossary_add_request::GlossaryAddRequest;
use crate::model::request::app::dict::word::glossary_request::GlossaryRequest;

pub fn glossary_query(request: &Json<GlossaryRequest>) -> Vec<UserDict> {
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
    let connection = config::establish_dict_connection();
    let timestamp: i64 = get_current_millisecond();
    let new_glossary = CustomUserDict {
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



