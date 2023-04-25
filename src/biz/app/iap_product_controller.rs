use okapi::openapi3::OpenApi;
use rocket::response::content;
use rocket::serde::json::Json;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_okapi::settings::OpenApiSettings;
use rust_wheel::common::util::model_convert::box_rest_response;

use crate::model::diesel::dolphin::dolphin_models::{IapProduct};
use crate::model::request::app::iap_product_request::IapProductRequest;
use crate::model::request::app::overview::product::add_product_request::AddProductRequest;
use crate::model::request::app::overview::product::edit_product_request::EditProductRequest;
use crate::service::app::iap_product_service::{iap_product_create, iap_product_detail, iap_product_query, iap_product_query_list};
use crate::service::app::product_service::{ product_edit};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: page, list, add, edit, get]
}

#[openapi(tag = "商品")]
#[post("/v1/page",data = "<request>")]
pub fn page(request: Json<IapProductRequest>) -> content::RawJson<String> {
    let products = iap_product_query::<Vec<IapProduct>>(&request);
    return box_rest_response(products);
}

#[openapi(tag = "商品")]
#[get("/v1/list")]
pub fn list() -> content::RawJson<String> {
    let products = iap_product_query_list::<Vec<IapProduct>>();
    return box_rest_response(products);
}

#[openapi(tag = "商品")]
#[post("/v1/add",data = "<request>")]
pub fn add(request: Json<AddProductRequest>) -> content::RawJson<String> {
    iap_product_create(&request);
    return box_rest_response("ok");
}

#[openapi(tag = "商品")]
#[patch("/v1/edit",data = "<request>")]
pub fn edit(request: Json<EditProductRequest>) -> content::RawJson<String> {
    product_edit(&request);
    return box_rest_response("ok");
}

#[openapi(tag = "商品")]
#[put("/v1/detail/<id>")]
pub fn get(id: i32) -> content::RawJson<String> {
    let app = iap_product_detail(id);
    return box_rest_response(app);
}