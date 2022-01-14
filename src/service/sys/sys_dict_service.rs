use diesel::ExpressionMethods;
use rocket::serde::json::Json;
use crate::model::diesel::quark::quark_models::SysDict;
use crate::diesel::prelude::*;
use rust_wheel::config::db::config;

pub fn dict_query<T>() -> Vec<SysDict> {
    use crate::model::diesel::quark::quark_schema::sys_dict::dsl::*;
    let connection = config::establish_dict_connection();
    let query = sys_dict.filter(id.gt(0))
        .limit(1)
        .load::<SysDict>(&connection)
        .expect("query dict content failed");
    return query;
}





