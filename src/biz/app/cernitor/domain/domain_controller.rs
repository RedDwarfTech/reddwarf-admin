use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::box_rest_response;
use crate::model::diesel::dolphin::dolphin_models::{Domain};
use crate::model::request::app::cernitor::domain::domain_request::DomainRequest;
use crate::service::app::cernitor::domain::domain_service::domain_query;

#[post("/v1/page", data = "<request>")]
pub fn page(request: Json<DomainRequest>) -> content::Json<String> {
    let domains = domain_query::<Vec<Domain>>(&request);
    return box_rest_response(domains);
}