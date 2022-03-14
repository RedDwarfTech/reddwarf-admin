// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]

use rocket::serde::Serialize;
use serde::Deserialize;

use crate::model::diesel::dict::dict_schema::*;

#[derive(Insertable,Queryable,Debug,Serialize,Deserialize,Default)]
#[table_name = "dict"]
pub struct Dict {
    pub word: String,
    pub phonetic: String,
    pub definition: String,
    pub translation: String,
    pub pos: String,
    pub collins: Option<i32>,
    pub oxford: Option<i32>,
    pub tag: String,
    pub bnc: i32,
    pub frq: i32,
    pub exchange: String,
    pub detail: String,
    pub audio: String,
    pub id: i64,
    pub demo_sentence: i32,
    pub sentence_count: Option<i32>,
}

#[derive(Insertable,Queryable,Debug,Serialize,Deserialize,Default)]
#[table_name = "user_dict"]
pub struct UserDict {
    pub id: i64,
    pub word_id: i64,
    pub user_id: i64,
    pub created_time: i64,
    pub updated_time: i64,
    pub status: i32,
    pub word: String,
}

