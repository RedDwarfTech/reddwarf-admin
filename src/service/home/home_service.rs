use rust_wheel::common::util::collection_util::take;
use rust_wheel::config::db::config;

use crate::diesel::prelude::*;
use crate::model::diesel::dolphin::dolphin_models::{Dashboard, Trend};

pub fn overview_query() -> Dashboard {
    use crate::model::diesel::dolphin::dolphin_schema::dashboard::dsl::*;
    let connection = config::establish_connection();
    let dashboards = dashboard.limit(1)
        .load::<Dashboard>(&connection)
        .expect("load dashboard failed");
    let dashboard_data = take(dashboards,0).unwrap();
    return dashboard_data;
}

pub fn trend_query() -> Vec<Trend> {
    use crate::model::diesel::dolphin::dolphin_schema::trend::dsl::*;
    let connection = config::establish_connection();
    let trends = trend
        .load::<Trend>(&connection)
        .expect("load dashboard failed");
    return trends;
}



