use diesel::ExpressionMethods;
use rust_wheel::config::db::config;

use crate::diesel::prelude::*;
use crate::model::diesel::quark::quark_models::SysDict;

pub fn dict_query<T>() -> Vec<SysDict> {
    use crate::model::diesel::quark::quark_schema::sys_dict::dsl::*;
    let connection = config::establish_quark_connection();
    let query = sys_dict.filter(id.gt(0))
        .limit(2000)
        .load::<SysDict>(&connection)
        .expect("query dict content failed");
    return query;
}

pub fn dict_page_query<T>() -> Vec<SysDict> {
    use crate::model::diesel::quark::quark_schema::sys_dict::dsl::*;
    let connection = config::establish_quark_connection();
    let query = sys_dict.filter(id.gt(0))
        .limit(2000)
        .load::<SysDict>(&connection)
        .expect("query dict content failed");
    return query;
}
