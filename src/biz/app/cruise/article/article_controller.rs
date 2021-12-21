use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::model::response::api_response::ApiResponse;
use rust_wheel::model::response::pagination_response::PaginationResponse;
use crate::model::request::app::cruise::article::article_request::ArticleRequest;
use crate::models::Favorites;
use crate::service::app::cruise::article::article_service::article_query;

#[post("/v1/page",data = "<request>")]
pub fn page(request: Json<ArticleRequest>) -> content::Json<String> {
    let fav_musics = article_query::<Vec<Favorites>>(&request);
    let source_ids: Vec<String> = fav_musics.list
        .iter()
        .map(|item| item.source_id.to_string())
        .collect();

    let resp = PaginationResponse{
        pagination: fav_musics.pagination,
        list: source_ids
    };
    let res = ApiResponse {
        result: resp,
        ..Default::default()
    };
    let response_json = serde_json::to_string(&res).unwrap();
    return content::Json(response_json);
}



