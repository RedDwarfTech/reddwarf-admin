use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::box_rest_response;

use crate::model::diesel::dolphin::dolphin_models::Article;
use crate::model::request::app::cruise::article::article_request::ArticleRequest;
use crate::service::app::cruise::article::article_service::{article_detail_query, article_query};

#[post("/v1/page", data = "<request>")]
pub fn page(request: Json<ArticleRequest>) -> content::RawJson<String> {
    let musics = article_query::<Vec<Article>>(&request);
    return box_rest_response(musics);
}

#[get("/v1/detail/<id>")]
pub fn detail(id: i64) -> content::RawJson<String> {
    let article = article_detail_query(id);
    return box_rest_response(article);
}





