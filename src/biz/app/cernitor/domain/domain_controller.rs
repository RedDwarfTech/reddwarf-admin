use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::box_rest_response;
use rust_wheel::model::user::login_user_info::LoginUserInfo;
use crate::model::diesel::dolphin::dolphin_models::{Domain};
use crate::model::request::app::cernitor::domain::add_domain_request::AddDomainRequest;
use crate::model::request::app::cernitor::domain::domain_request::DomainRequest;
use crate::service::app::cernitor::domain::domain_service::{add_domain, domain_query};

#[post("/v1/page", data = "<request>")]
pub fn page(request: Json<DomainRequest>) -> content::Json<String> {
    let domains_info = domain_query::<Vec<Domain>>(&request);
    return box_rest_response(domains_info);
}

#[post("/v1/add", data = "<request>")]
pub fn add(request: Json<AddDomainRequest>, login_user_info: LoginUserInfo) -> content::Json<String> {
    add_domain(&request, login_user_info);
    return box_rest_response("ok");
}