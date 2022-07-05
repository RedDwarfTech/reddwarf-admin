use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::box_rest_response;
use crate::model::request::app::overview::product::add_product_request::AddProductRequest;
use crate::model::request::app::overview::product::edit_product_request::EditProductRequest;
use crate::model::diesel::dolphin::dolphin_models::Product;
use crate::model::request::app::product_request::ProductRequest;
use crate::service::app::product_service::{product_create, product_detail, product_edit, product_query, product_query_list};

#[post("/v1/page",data = "<request>")]
pub fn page(request: Json<ProductRequest>) -> content::RawJson<String> {
    let products = product_query::<Vec<Product>>(&request);
    return box_rest_response(products);
}

#[get("/v1/list")]
pub fn list() -> content::RawJson<String> {
    let products = product_query_list::<Vec<Product>>();
    return box_rest_response(products);
}

#[post("/v1/add",data = "<request>")]
pub fn add(request: Json<AddProductRequest>) -> content::RawJson<String> {
    product_create(&request);
    return box_rest_response("ok");
}

#[patch("/v1/edit",data = "<request>")]
pub fn edit(request: Json<EditProductRequest>) -> content::RawJson<String> {
    product_edit(&request);
    return box_rest_response("ok");
}

#[put("/v1/detail/<id>")]
pub fn get(id: i32) -> content::RawJson<String> {
    let app = product_detail(id);
    return box_rest_response(app);
}