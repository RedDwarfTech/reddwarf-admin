use okapi::openapi3::OpenApi;
use rocket::response::content;
use rocket::serde::json::Json;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_okapi::settings::OpenApiSettings;
use rust_wheel::common::wrapper::rocket_http_resp::box_rest_response;
use crate::model::diesel::dolphin::dolphin_models::Product;
use crate::model::request::app::overview::product::add_product_request::AddProductRequest;
use crate::model::request::app::overview::product::edit_product_request::EditProductRequest;
use crate::model::request::app::product_request::ProductRequest;
use crate::service::app::product_service::{product_create, product_detail, product_edit, product_query, product_query_list};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: page, list, add, edit, get]
}

#[openapi(tag = "产品")]
#[post("/v1/page",data = "<request>")]
pub fn page(request: Json<ProductRequest>) -> content::RawJson<String> {
    let products = product_query::<Vec<Product>>(&request);
    return box_rest_response(products);
}

#[openapi(tag = "产品")]
#[get("/v1/list")]
pub fn list() -> content::RawJson<String> {
    let products = product_query_list::<Vec<Product>>();
    return box_rest_response(products);
}

#[openapi(tag = "产品")]
#[post("/v1/add",data = "<request>")]
pub fn add(request: Json<AddProductRequest>) -> content::RawJson<String> {
    product_create(&request);
    return box_rest_response("ok");
}

#[openapi(tag = "产品")]
#[patch("/v1/edit",data = "<request>")]
pub fn edit(request: Json<EditProductRequest>) -> content::RawJson<String> {
    product_edit(&request);
    return box_rest_response("ok");
}

#[openapi(tag = "产品")]
#[put("/v1/detail/<id>")]
pub fn get(id: i32) -> content::RawJson<String> {
    let app = product_detail(id);
    return box_rest_response(app);
}