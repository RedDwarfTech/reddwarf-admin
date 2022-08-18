use okapi::openapi3::OpenApi;
use rocket::response::content;
use rocket::serde::json::Json;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_okapi::settings::OpenApiSettings;
use rust_wheel::common::util::model_convert::{box_rest_response, box_type_rest_response};
use rust_wheel::model::response::api_response::ApiResponse;

use crate::model::diesel::dolphin::dolphin_models::Article;
use crate::model::request::app::cruise::article::article_request::ArticleRequest;
use crate::model::response::app::cruise::article::article_response::ArticleResponse;
use crate::service::app::cruise::article::article_service::{article_detail_query, article_query};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: page, detail]
}

#[openapi(tag = "文章")]
#[post("/v1/page", data = "<request>")]
pub fn page(request: Json<ArticleRequest>) -> content::RawJson<String> {
    let musics = article_query::<Vec<Article>>(&request);
    return box_rest_response(musics);
}

#[openapi(tag = "文章")]
#[get("/v1/detail/<id>")]
pub fn detail(id: i64) -> Json<ApiResponse<ArticleResponse>> {
    let article = article_detail_query(id);
    let boxed_response = box_type_rest_response(article);
    return Json::from(boxed_response);
}





