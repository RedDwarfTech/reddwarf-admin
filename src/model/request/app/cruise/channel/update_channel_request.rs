use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
#[allow(non_snake_case)]
pub struct UpdateChannelRequest {
    pub channelId: i64,
    pub tags: Option<Vec<Tag>>,
    pub commentRss: Option<i32>,
    pub partOutput: Option<i32>,
    pub subStatus: Option<i32>
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
#[allow(non_snake_case)]
pub struct Tag {
    pub code: String
}