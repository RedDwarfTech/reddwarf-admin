use rocket::serde::json::Json;
use rust_wheel::config::db::config;
use crate::models::{Dashboard};
use crate::diesel::prelude::*;
use crate::model::diesel::dolphin::dolphin_schema::dashboard::dsl::dashboard;
use crate::model::request::user::login_request::LoginRequest;

pub fn login_impl(request: Json<LoginRequest>) -> Vec<Dashboard>{
    use crate::model::diesel::dolphin::dolphin_schema::dashboard::dsl::*;
    let connection = config::establish_connection();
    let results = dashboard
        .limit(1)
        .load::<Dashboard>(&connection)
        .expect("Error loading home");
    return results;
}


