table! {
    article_content (id) {
        id -> Int8,
        article_id -> Int8,
        #[sql_name = "article_content"]
        articleContent -> Varchar,
    }
}