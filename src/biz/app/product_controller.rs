use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::box_rest_response;

use crate::model::diesel::dolphin::dolphin_models::{App, Product};
use crate::model::request::app::add_app_request::AddAppRequest;
use crate::model::request::app::edit_app_request::EditAppRequest;
use crate::model::request::app::product_request::ProductRequest;
use crate::service::app::app_service::{app_create, app_detail, app_edit};
use crate::service::app::product_service::product_query;

#[post("/v1/page",data = "<request>")]
pub fn page(request: Json<ProductRequest>) -> content::Json<String> {
    let products = product_query::<Vec<Product>>(&request);
    return box_rest_response(products);
}

#[post("/v1/add",data = "<request>")]
pub fn add(request: Json<AddAppRequest>) -> content::Json<String> {
    app_create(&request);
    return box_rest_response("ok");
}

#[put("/v1/edit",data = "<request>")]
pub fn edit(request: Json<EditAppRequest>) -> content::Json<String> {
    app_edit(&request);
    return box_rest_response("ok");
}

#[put("/v1/detail/<id>")]
pub fn get(id: i32) -> content::Json<String> {
    let app = app_detail(id);
    return box_rest_response(app);
}