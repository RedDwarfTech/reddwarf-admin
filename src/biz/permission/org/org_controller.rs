use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::box_rest_response;

use crate::model::diesel::dolphin::dolphin_models::MenuResource;
use crate::model::request::permission::menu::menu_request::MenuRequest;
use crate::model::request::user::password_request::PasswordRequest;
use crate::model::response::permission::org::org_response::OrgResponse;
use crate::service::permission::org::org_service::{org_edit, org_query_full_tree, org_query_tree};

#[post("/v1/page",data = "<request>")]
pub fn page_tree(request: Json<MenuRequest>) -> content::Json<String> {
    let roles = org_query_tree::<Vec<OrgResponse>>(&request);
    return box_rest_response(roles);
}

#[post("/v1/tree",data = "<request>")]
pub fn org_tree(request: Json<MenuRequest>) -> content::Json<String> {
    let roles = org_query_full_tree::<Vec<OrgResource>>(request.parentId);
    return box_rest_response(roles);
}

#[post("/v1/org/edit",data = "<request>")]
pub fn edit_org(request: Json<PasswordRequest>) -> content::Json<String> {
    return org_edit(&request);
}
