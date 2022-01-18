use rocket::serde::Deserialize;
use rocket::serde::Serialize;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct ChannelRequest {
    pub userId: Option<i64>,
    pub pageNum: Option<i64>,
    pub pageSize: Option<i64>,
    pub editorPick: Option<i32>
}
