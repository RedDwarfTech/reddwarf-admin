use diesel::{ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl};
use diesel::dsl::{any};
use diesel::query_builder::BoxedSelectStatement;
use diesel_full_text_search::{to_tsquery, to_tsvector, TsVectorExtensions};
use rocket::serde::json::Json;
use rust_wheel::common::query::pagination_pg_big_table::PaginateForPgBigTableQueryFragment;
use rust_wheel::common::util::collection_util::take;
use rust_wheel::common::util::model_convert::map_pagination_from_list;
use rust_wheel::config::db::config;
use rust_wheel::model::response::pagination_response::PaginationResponse;

use crate::model::diesel::dolphin::dolphin_models::{Article, ArticleContent, RssSubSource};
use crate::model::request::app::cruise::article::article_request::ArticleRequest;
use crate::model::response::app::cruise::article::article_response::ArticleResponse;

pub fn article_query<T>(request: &Json<ArticleRequest>) -> PaginationResponse<Vec<ArticleResponse>> {
    // when pagination with the big table
    // using the estimate rows not the precise row count to speed the query
    use crate::model::diesel::dolphin::dolphin_schema::article as article_table;
    let mut query:BoxedSelectStatement<(diesel::sql_types::BigInt, diesel::sql_types::BigInt, diesel::sql_types::Text, diesel::sql_types::Text, diesel::sql_types::Text, diesel::sql_types::BigInt, diesel::sql_types::BigInt, diesel::sql_types::Nullable<diesel::sql_types::Text>, diesel::sql_types::Nullable<diesel::pg::types::sql_types::Timestamptz>, diesel::sql_types::BigInt, diesel::sql_types::Nullable<diesel::sql_types::Text>, diesel::sql_types::Integer, diesel::sql_types::Nullable<diesel::sql_types::Integer>), article_table::table, diesel::pg::Pg> = article_table::table.into_boxed::<diesel::pg::Pg>();
    if let Some(max_offset) = &request.maxOffset {
        query = query.filter(article_table::id.lt(max_offset));
    }
    if let Some(req_channel_id) = &request.channelId {
        query = query.filter(article_table::sub_source_id.eq(req_channel_id));
    }
    if let Some(filter_title) = &request.title {
        let query_items: Vec<&str> = filter_title.trim().split_whitespace().collect();
        let query_array = query_items.join(" & ");
        let tsquery = to_tsquery(query_array);
        let tsvector = to_tsvector("'dolphinzhcfg', title");
        query = query.filter(tsvector.matches(tsquery));
        return get_query_result(query,request);
    }
    return get_query_result(query,request);
}

pub fn get_query_result(query:BoxedSelectStatement<(diesel::sql_types::BigInt, diesel::sql_types::BigInt, diesel::sql_types::Text, diesel::sql_types::Text, diesel::sql_types::Text, diesel::sql_types::BigInt, diesel::sql_types::BigInt, diesel::sql_types::Nullable<diesel::sql_types::Text>, diesel::sql_types::Nullable<diesel::pg::types::sql_types::Timestamptz>, diesel::sql_types::BigInt, diesel::sql_types::Nullable<diesel::sql_types::Text>, diesel::sql_types::Integer, diesel::sql_types::Nullable<diesel::sql_types::Integer>), crate::model::diesel::dolphin::dolphin_schema::article::table, diesel::pg::Pg>,request: &Json<ArticleRequest>) -> PaginationResponse<Vec<ArticleResponse>> {
    use crate::model::diesel::dolphin::dolphin_schema::article::dsl::*;
    let connection = config::establish_connection();
    let query = query
        .order(created_time.desc())
        .paginate_pg_big_table(request.pageNum, "article".parse().unwrap())
        .per_page(request.pageSize);
    let query_result: QueryResult<(Vec<_>, i64, i64)> = query.pg_big_table_load_and_count_pages_total::<Article>(&connection);
    let article_response = append_channel_name(&query_result.as_ref().unwrap().0, &connection);
    let total = query_result.as_ref().unwrap().2;
    let page_result = map_pagination_from_list(article_response, request.pageNum, request.pageSize, total);
    return page_result;
}

pub fn append_channel_name(articles: &Vec<Article>, connection:&PgConnection) -> Vec<ArticleResponse>{
    use crate::model::diesel::dolphin::dolphin_schema::rss_sub_source::dsl::*;
    let channel_ids:Vec<i64> = articles.iter()
        .map(|item| item.sub_source_id)
        .collect();
    let channels = rss_sub_source.filter(id.eq(any(channel_ids)))
        .load::<RssSubSource>(connection)
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
        .load::<Article>(&connection)
        .expect("query article id failed");
    let article_result:Article = take(query,0).unwrap();
    let contents = article_content.filter(crate::model::diesel::dolphin::dolphin_schema::article_content::article_id.eq(filter_article_id))
        .limit(1)
        .load::<ArticleContent>(&connection)
        .expect("query article content failed");
    let single_content = take(contents,0).unwrap();
    let mut article_response = ArticleResponse::new(&article_result, single_content);
    let channel_predicate = crate::model::diesel::dolphin::dolphin_schema::rss_sub_source::dsl::id.eq(&article_result.sub_source_id);
    // https://stackoverflow.com/questions/46297867/how-do-i-get-an-optiont-instead-of-an-optionvect-from-a-diesel-query-which
    let channel = rss_sub_source.filter(channel_predicate)
        .first::<RssSubSource>(&connection)
        .optional()
        .unwrap();
    article_response.channel_name = channel.unwrap().sub_name;
    return article_response;
}

pub fn get_article_count_by_channel_id(req_channel_id: &i64) -> i64 {
    use crate::model::diesel::dolphin::dolphin_schema::article::dsl::*;
    let connection = config::establish_connection();
    let predicate = crate::model::diesel::dolphin::dolphin_schema::article::sub_source_id.eq(req_channel_id);
    let query = article
        .filter(predicate);
    let query_result = query.count().get_result(&connection);
    return query_result.unwrap_or(0);
}


