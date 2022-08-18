use okapi::openapi3::OpenApi;
use rocket::response::content;
use rocket::serde::json::Json;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_okapi::settings::OpenApiSettings;
use rust_wheel::common::util::model_convert::box_rest_response;

use crate::model::request::permission::org::org_request::OrgRequest;
use crate::model::request::user::password_request::PasswordRequest;
use crate::model::response::permission::org::org_response::OrgResponse;
use crate::service::permission::org::org_service::{org_edit, org_list, org_query_full_tree, org_query_tree};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: page_tree, org_tree, edit_org, org_list_query]
}

#[openapi(tag = "机构")]
#[post("/v1/page",data = "<request>")]
pub fn page_tree(request: Json<OrgRequest>) -> content::RawJson<String> {
    let roles = org_query_tree::<Vec<OrgResponse>>(&request);
    return box_rest_response(roles);
}

#[openapi(tag = "机构")]
#[post("/v1/tree",data = "<request>")]
pub fn org_tree(request: Json<OrgRequest>) -> content::RawJson<String> {
    let roles = org_query_full_tree::<Vec<OrgResponse>>(request.parentId);
    return box_rest_response(roles);
}

#[openapi(tag = "机构")]
#[post("/v1/org/edit",data = "<request>")]
pub fn edit_org(request: Json<PasswordRequest>) -> content::RawJson<String> {
    return org_edit(&request);
}

#[openapi(tag = "机构")]
#[get("/v1/org/list")]
pub fn org_list_query() -> content::RawJson<String> {
    let org_result = org_list();
    return box_rest_response(org_result);
}