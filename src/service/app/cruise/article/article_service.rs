use diesel::{ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl};
use diesel::dsl::any;
use rocket::serde::json::Json;
use rust_wheel::common::query::pagination::Paginate;
use rust_wheel::common::util::collection_util::take;
use rust_wheel::common::util::model_convert::map_pagination_from_list;
use rust_wheel::config::db::config;
use rust_wheel::model::response::pagination_response::PaginationResponse;
use diesel::internal::table_macro::BoxedSelectStatement;
use crate::common::db::database::{get_conn};
use crate::model::diesel::dolphin::dolphin_models::{Article, ArticleContent, RssSubSource};
use crate::model::request::app::cruise::article::article_request::ArticleRequest;
use crate::model::response::app::cruise::article::article_response::ArticleResponse;
use diesel::internal::table_macro::FromClause;

type QueryType<'a> = BoxedSelectStatement<'a, (diesel::sql_types::BigInt, diesel::sql_types::BigInt, diesel::sql_types::Text, diesel::sql_types::Text, diesel::sql_types::Text, diesel::sql_types::BigInt, diesel::sql_types::BigInt, diesel::sql_types::Nullable<diesel::sql_types::Text>, diesel::sql_types::Nullable<diesel::pg::sql_types::Timestamptz>, diesel::sql_types::BigInt, diesel::sql_types::Nullable<diesel::sql_types::Text>, diesel::sql_types::Integer, diesel::sql_types::Nullable<diesel::sql_types::Integer>,diesel::sql_types::SmallInt), FromClause<crate::model::diesel::dolphin::dolphin_schema::article::table>, diesel::pg::Pg>;

pub fn article_query<T>(request: &Json<ArticleRequest>) -> PaginationResponse<Vec<ArticleResponse>> {
    use crate::model::diesel::dolphin::dolphin_schema::article as article_table;   
    let mut query: QueryType = article_table::table.into_boxed::<diesel::pg::Pg>();
    if let Some(max_offset) = &request.maxOffset {
        query = query.filter(article_table::id.lt(max_offset));
    }
    if let Some(req_channel_id) = &request.channelId {
        query = query.filter(article_table::sub_source_id.eq(req_channel_id));
    }
    if let Some(filter_title) = &request.title {
        if !filter_title.trim().is_empty() {
            return get_full_text_search_result(filter_title);
        }
    }
    return get_query_result(query,request);
}

/// https://stackoverflow.com/questions/21747136/how-do-i-print-the-type-of-a-variable
/// when pagination with the big table
/// using the estimate rows not the precise row count to speed the query
pub fn get_full_text_search_result(filter_title: &str) -> PaginationResponse<Vec<ArticleResponse>>{
    let connection = config::establish_connection();
    let query_items: Vec<&str> = filter_title.trim().split_whitespace().collect();
    let query_array = query_items.join("|");
    let results = diesel::sql_query(format!("SELECT * FROM article WHERE to_tsvector('dolphinzhcfg', title) @@ plainto_tsquery('{}') limit 10 offset 0",query_array))
        .load::<Article>(&mut get_conn())
        .expect("An error has occured");
    let article_response = append_channel_name(&results, &mut get_conn());
    let page_result = map_pagination_from_list(article_response, 1, 10, 15);
    return page_result;

}

pub fn get_query_result(query: QueryType,request: &Json<ArticleRequest>) -> PaginationResponse<Vec<ArticleResponse>> {
    use crate::model::diesel::dolphin::dolphin_schema::article::dsl::*;
    let connection = config::establish_connection();
    let query = query
        .order(created_time.desc())
        .paginate(request.pageNum)
        .per_page(request.pageSize);
    // let sql1 = debug_query::<diesel::pg::Pg, _>(&query);
    // println!("sql:{}",sql1);
    let query_result: QueryResult<(Vec<_>, i64)> = query.load_and_count_pages::<Article>(&mut get_conn());
    let article_response = append_channel_name(&query_result.as_ref().unwrap().0, &mut get_conn());
    let total = 1000;
    let page_result = map_pagination_from_list(article_response, request.pageNum, request.pageSize, total);
    return page_result;
}

pub fn append_channel_name(articles: &Vec<Article>, connection:&PgConnection) -> Vec<ArticleResponse>{
    use crate::model::diesel::dolphin::dolphin_schema::rss_sub_source::dsl::*;
    let channel_ids:Vec<i64> = articles.iter()
        .map(|item| item.sub_source_id)
        .collect();
    let channels = rss_sub_source.filter(id.eq(any(channel_ids)))
        .load::<RssSubSource>(&mut get_conn())
        .expect("query rss sub source failed");
    let mut article_res = Vec::new();
    for article in articles {
        let channel_name = channels.iter().filter(|channel|channel.id==article.sub_source_id)
            .map(|channel|channel.sub_name.to_string())
            .collect::<String>();
        let mut article_response = ArticleResponse::from(article);
        article_response.channel_name = channel_name;
        article_res.push(article_response);
    }
    return article_res;
}

pub fn article_detail_query(filter_article_id: i64) -> ArticleResponse {
    use crate::diesel::OptionalExtension;
    use crate::model::diesel::dolphin::dolphin_schema::article::dsl::*;
    use crate::model::diesel::dolphin::dolphin_schema::article_content::dsl::*;
    use crate::model::diesel::dolphin::dolphin_schema::rss_sub_source::dsl::*;
    let connection = config::establish_connection();
    let query = article.filter(crate::model::diesel::dolphin::dolphin_schema::article::id.eq(filter_article_id))
        .limit(1)
        .load::<Article>(&mut get_conn())
        .expect("query article id failed");
    let article_result:Article = take(query,0).unwrap();
    let contents = article_content.filter(crate::model::diesel::dolphin::dolphin_schema::article_content::article_id.eq(filter_article_id))
        .limit(1)
        .load::<ArticleContent>(&mut get_conn())
        .expect("query article content failed");
    let single_content = take(contents,0).unwrap();
    let mut article_response = ArticleResponse::new(&article_result, single_content);
    let channel_predicate = crate::model::diesel::dolphin::dolphin_schema::rss_sub_source::dsl::id.eq(&article_result.sub_source_id);
    // https://stackoverflow.com/questions/46297867/how-do-i-get-an-optiont-instead-of-an-optionvect-from-a-diesel-query-which
    let channel = rss_sub_source.filter(channel_predicate)
        .first::<RssSubSource>(&mut get_conn())
        .optional()
        .unwrap();
    article_response.channel_name = channel.as_ref().unwrap().sub_name.to_string();
    article_response.sub_url = channel.as_ref().unwrap().sub_url.to_string();
    return article_response;
}

pub fn get_article_count_by_channel_id(req_channel_id: &i64) -> i64 {
    use crate::model::diesel::dolphin::dolphin_schema::article::dsl::*;
    let connection = config::establish_connection();
    let predicate = crate::model::diesel::dolphin::dolphin_schema::article::sub_source_id.eq(req_channel_id);
    let query = article
        .filter(predicate);
    let query_result = query.count().get_result(&mut get_conn());
    return query_result.unwrap_or(0);
}


