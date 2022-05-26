use rocket::serde::Deserialize;
use rocket::serde::Serialize;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct ArticleRequest {
    pub userId: Option<i64>,
    pub pageNum: i64,
    pub pageSize: i64,
    pub maxOffset: Option<i64>,
    pub channelId: Option<i64>,
    pub title: Option<String>
}