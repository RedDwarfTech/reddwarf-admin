use chrono::{DateTime, Utc};
use rocket::serde::Deserialize;
use rocket::serde::Serialize;

use crate::model::diesel::dolphin::dolphin_models::Article;
use crate::model::diesel::dolphin::dolphin_models::ArticleContent;

#[derive( Serialize, Queryable, Deserialize,Default, Clone)]
pub struct ArticleResponse {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub author: String,
    pub guid: String,
    pub created_time: i64,
    pub updated_time: i64,
    pub link: Option<String>,
    pub pub_time: Option<DateTime<Utc>>,
    pub sub_source_id: i64,
    pub cover_image: Option<String>,
    pub channel_reputation: i32,
    pub editor_pick: Option<i32>,
    pub content: String,
    pub channel_name: String
}

impl ArticleResponse {
    pub(crate) fn new(article: Article, article_content: ArticleContent) -> Self {
        Self {
            id: article.id,
            user_id: 0,
            title: article.title,
            author: article.author,
            guid: "".to_string(),
            created_time: article.created_time,
            updated_time: article.updated_time,
            link: None,
            pub_time: None,
            sub_source_id: article.sub_source_id,
            cover_image: None,
            channel_reputation: 0,
            editor_pick: None,
            content: article_content.content,
            channel_name: "".to_string()
        }
    }
}

impl From<&Article> for ArticleResponse {
    fn from(p: &Article) -> Self {
        Self {
            id: p.id,
            user_id: 0,
            title: p.title.to_string(),
            author: p.author.to_string(),
            guid: "".to_string(),
            created_time: p.created_time,
            updated_time: 0,
            link: None,
            pub_time: None,
            sub_source_id: p.sub_source_id,
            cover_image: None,
            channel_reputation: 0,
            editor_pick: None,
            content: "".to_string(),
            channel_name: "".to_string()
        }
    }
}
