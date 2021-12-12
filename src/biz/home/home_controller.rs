use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::model::response::api_response::ApiResponse;
use rust_wheel::model::user::login_user_info::LoginUserInfo;
use crate::model::request::home::home_request::HomeRequest;
use crate::model::response::home::dashboard_response::DashboardResponse;
use crate::models::Dashboard;
use crate::service::home::home_service::dashboard_impl;

#[post("/v1/dashboard/overview",data = "<record>")]
pub fn overview(record: Json<HomeRequest>, login_user_info: LoginUserInfo) -> content::Json<String> {
    let dashboards = dashboard_impl();
    let dashboard_response = DashboardResponse{
        id: if dashboards.len()>0{dashboards[0].id}else{0},
        appCount: if dashboards.len()>0{dashboards[0].app_count}else{0},
        userCount: if dashboards.len()>0{dashboards[0].user_count}else{0}
    };
    let res = ApiResponse {
        result: dashboard_response,
        ..Default::default()
    };
    let response_json = serde_json::to_string(&res).unwrap();
    return content::Json(response_json);
}





