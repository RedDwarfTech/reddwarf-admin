use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use rocket::serde::json::Json;
use rust_wheel::common::query::pagination::PaginateForQueryFragment;
use rust_wheel::common::util::collection_util::take;
use rust_wheel::common::util::model_convert::map_pagination_res;
use rust_wheel::config::db::config;
use rust_wheel::model::response::pagination_response::PaginationResponse;

use crate::model::diesel::dolphin::custom_dolphin_models::ArticleContent;
use crate::model::diesel::dolphin::dolphin_models::Article;
use crate::model::request::app::cruise::article::article_request::ArticleRequest;
use crate::model::response::app::cruise::article::article_response::ArticleResponse;

pub fn article_query<T>(request: &Json<ArticleRequest>) -> PaginationResponse<Vec<Article>> {
    use crate::model::diesel::dolphin::dolphin_schema::article::dsl::*;
    let connection = config::establish_connection();
    let query = article
        .filter(id.gt(0))
        .order(created_time.desc())
        .paginate(request.pageNum)
        .per_page(request.pageSize);
    let query_result: QueryResult<(Vec<_>, i64, i64)> = query.load_and_count_pages_total::<Article>(&connection);
    let page_result = map_pagination_res(query_result, request.pageNum, request.pageSize);
    return page_result;
}

pub fn article_detail_query(filter_article_id: i64) -> ArticleResponse {
    use crate::model::diesel::dolphin::dolphin_schema::article::dsl::*;
    use crate::model::diesel::dolphin::custom_dolphin_schema::article_content::dsl::*;
    let connection = config::establish_connection();
    let query = article.filter(crate::model::diesel::dolphin::dolphin_schema::article::id.eq(filter_article_id))
        .limit(1)
        .load::<Article>(&connection)
        .expect("query article id failed");
    let article_result:Article = take(query,0).unwrap();
    let contents = article_content.filter(crate::model::diesel::dolphin::custom_dolphin_schema::article_content::article_id.eq(filter_article_id))
        .limit(1)
        .load::<ArticleContent>(&connection)
        .expect("query article content failed");
    let content = take(contents,0).unwrap();
    let article_response = ArticleResponse::new(article_result, content);
    return article_response;
}
