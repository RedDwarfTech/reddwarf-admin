use rust_wheel::common::util::collection_util::take;
use rust_wheel::config::db::config;

use crate::common::db::database::get_conn;
use crate::diesel::prelude::*;
use crate::model::diesel::dolphin::dolphin_models::{Dashboard, Trend};

pub fn overview_query() -> Dashboard {
    use crate::model::diesel::dolphin::dolphin_schema::dashboard::dsl::*;
    
    let dashboards = dashboard.limit(1)
        .load::<Dashboard>(&mut get_conn())
        .expect("load dashboard failed");
    let dashboard_data = take(dashboards,0).unwrap();
    return dashboard_data;
}

pub fn trend_query() -> Vec<Trend> {
    use crate::model::diesel::dolphin::dolphin_schema::trend::dsl::*;
    
    let trends = trend
        .load::<Trend>(&mut get_conn())
        .expect("load dashboard failed");
    return trends;
}



