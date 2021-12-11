use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::model::user::login_user_info::LoginUserInfo;
use crate::model::request::home::home_request::HomeRequest;

#[post("/v1/dashboard/overview",data = "<record>")]
pub fn overview(record: Json<HomeRequest>, login_user_info: LoginUserInfo) -> content::Json<String> {

    //let response_json = serde_json::to_string(&res).unwrap();
    return content::Json("".to_string());
}





