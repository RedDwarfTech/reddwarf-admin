use rocket::serde::Deserialize;
use rocket::serde::Serialize;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct PickChannelRequest {
    pub channelId: i64,
    pub editor_pick: i32,
}
