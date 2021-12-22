use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::model::response::api_response::ApiResponse;
use crate::model::diesel::dolphin::dolphin_models::Article;
use crate::model::request::app::cruise::article::article_request::ArticleRequest;
use crate::service::app::cruise::article::article_service::article_query;

#[post("/v1/page",data = "<request>")]
pub fn page(request: Json<ArticleRequest>) -> content::Json<String> {
    let musics = article_query::<Vec<Article>>(&request);
    let res = ApiResponse {
        result: musics,
        ..Default::default()
    };
    let response_json = serde_json::to_string(&res).unwrap();
    return content::Json(response_json);
}



