use rocket::serde::Deserialize;
use rocket::serde::Serialize;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct UpdateChannelRequest {
    pub channelId: i64,
    pub tags: Option<Vec<Tag>>
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Tag {
    pub code: String
}