use rocket::serde::Deserialize;
use rocket::serde::Serialize;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct ChannelRequest {
    pub userId: Option<i64>,
    pub pageNum: i64,
    pub pageSize: i64,
}

impl Default for ChannelRequest
{
    fn default() -> Self {
        ChannelRequest {
            userId: None,
            pageNum: 1,
            pageSize: 10,
        }
    }
}