use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::box_rest_response;

use crate::model::diesel::dolphin::dolphin_models::Product;
use crate::model::request::app::add_app_request::AddAppRequest;
use crate::model::request::app::edit_app_request::EditAppRequest;
use crate::model::request::app::product_request::ProductRequest;
use crate::service::app::product_service::{product_create, product_detail, product_edit, product_query};

#[post("/v1/page",data = "<request>")]
pub fn page(request: Json<ProductRequest>) -> content::RawJson<String> {
    let products = product_query::<Vec<Product>>(&request);
    return box_rest_response(products);
}

#[post("/v1/add",data = "<request>")]
pub fn add(request: Json<AddAppRequest>) -> content::RawJson<String> {
    product_create(&request);
    return box_rest_response("ok");
}

#[put("/v1/edit",data = "<request>")]
pub fn edit(request: Json<EditAppRequest>) -> content::RawJson<String> {
    product_edit(&request);
    return box_rest_response("ok");
}

#[put("/v1/detail/<id>")]
pub fn get(id: i32) -> content::RawJson<String> {
    let app = product_detail(id);
    return box_rest_response(app);
}