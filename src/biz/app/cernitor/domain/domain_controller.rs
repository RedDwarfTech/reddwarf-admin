use regex::Regex;
use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::{box_error_rest_response, box_rest_response};
use rust_wheel::model::user::login_user_info::LoginUserInfo;

use crate::model::diesel::dolphin::dolphin_models::Domain;
use crate::model::request::app::cernitor::domain::add_domain_request::AddDomainRequest;
use crate::model::request::app::cernitor::domain::domain_request::DomainRequest;
use crate::model::request::app::cernitor::domain::edit_domain_request::EditDomainRequest;
use crate::service::app::cernitor::domain::domain_service::{add_domain, domain_query, edit_domain};

#[post("/v1/page", data = "<request>")]
pub fn page(request: Json<DomainRequest>) -> content::Json<String> {
    let domains_info = domain_query::<Vec<Domain>>(&request);
    return box_rest_response(domains_info);
}

#[post("/v1/add", data = "<request>")]
pub fn add(request: Json<AddDomainRequest>, login_user_info: LoginUserInfo) -> content::Json<String> {
    let domain_regex = Regex::new(r"").unwrap();
    if !domain_regex.is_match(&request.domainUrl) {
        return box_error_rest_response("failed", "00100100064008".parse().unwrap(), "domain url format error".parse().unwrap());
    }
    add_domain(&request, login_user_info);
    return box_rest_response("ok");
}

#[post("/v1/edit", data = "<request>")]
pub fn edit(request: Json<EditDomainRequest>, login_user_info: LoginUserInfo) -> content::Json<String> {
    let domain_regex = Regex::new(r"").unwrap();
    if !domain_regex.is_match(&request.domainUrl) {
        return box_error_rest_response("failed", "00100100064008".parse().unwrap(), "domain url format error".parse().unwrap());
    }
    edit_domain(&request, login_user_info);
    return box_rest_response("ok");
}