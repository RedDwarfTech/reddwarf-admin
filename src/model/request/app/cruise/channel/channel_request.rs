use rocket::serde::Deserialize;
use rocket::serde::Serialize;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub enum ChannelRequest {
    userId(Option<i64>),
    pageNum(Option<i64>),
    pageSize(Option<i64>),
    editorPick(Option<i32>),
}
