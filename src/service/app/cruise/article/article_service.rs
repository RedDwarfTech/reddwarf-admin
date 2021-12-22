use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use rocket::serde::json::Json;
use rust_wheel::common::query::pagination::PaginateForQueryFragment;
use rust_wheel::common::util::model_convert::map_pagination_res;
use rust_wheel::model::response::pagination_response::PaginationResponse;
use crate::model::request::app::cruise::article::article_request::ArticleRequest;
use rust_wheel::config::db::config;
use crate::model::diesel::dolphin::dolphin_models::Article;

pub fn article_query<T>(request: &Json<ArticleRequest>) -> PaginationResponse<Vec<Article>> {
    use crate::model::diesel::dolphin::dolphin_schema::article::dsl::*;
    let connection = config::establish_connection();
    let query = article
        .filter(editor_pick.eq(1))
        .paginate(request.pageNum)
        .per_page(request.pageSize);
    let query_result: QueryResult<(Vec<_>, i64, i64)> = query.load_and_count_pages_total::<Article>(&connection);
    let page_result = map_pagination_res(query_result, request.pageNum, request.pageSize);
    return page_result;
}

pub fn article_detail_query(article_id: i64) -> Vec<Article> {
    use crate::model::diesel::dolphin::dolphin_schema::article::dsl::*;
    let connection = config::establish_connection();
    let query = article.filter(id.eq(article_id))
        .limit(1)
        .load::<Article>(&connection)
        .expect("query article id failed");
    return query;
}
