use rust_wheel::config::db::config;
use crate::models::{Dashboard};
use crate::diesel::prelude::*;

pub fn word_search_impl() -> Vec<Dashboard>{
    use crate::schema::dashboard::dsl::*;
    let connection = config::establish_connection();
    let results = dashboard
        .limit(1)
        .load::<Dashboard>(&connection)
        .expect("Error loading home");
    return results;
}


