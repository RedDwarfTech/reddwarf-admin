use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::box_rest_response;

use crate::model::request::permission::org::org_request::OrgRequest;
use crate::model::request::user::password_request::PasswordRequest;
use crate::model::response::permission::org::org_response::OrgResponse;
use crate::service::permission::org::org_service::{org_edit, org_list, org_query_full_tree, org_query_tree};

#[post("/v1/page",data = "<request>")]
pub fn page_tree(request: Json<OrgRequest>) -> content::RawJson<String> {
    let roles = org_query_tree::<Vec<OrgResponse>>(&request);
    return box_rest_response(roles);
}

#[post("/v1/tree",data = "<request>")]
pub fn org_tree(request: Json<OrgRequest>) -> content::RawJson<String> {
    let roles = org_query_full_tree::<Vec<OrgResponse>>(request.parentId);
    return box_rest_response(roles);
}

#[post("/v1/org/edit",data = "<request>")]
pub fn edit_org(request: Json<PasswordRequest>) -> content::RawJson<String> {
    return org_edit(&request);
}

#[get("/v1/org/list")]
pub fn org_list_query() -> content::RawJson<String> {
    let org_result = org_list();
    return box_rest_response(org_result);
}