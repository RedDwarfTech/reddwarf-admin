use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;

#[derive(Debug, PartialEq, Eq, Deserialize, FromForm, Serialize, JsonSchema)]
#[allow(non_snake_case)]
pub struct ChannelRequest {
    pub userId: Option<i64>,
    pub id: Option<i64>,
    pub pageNum: Option<i64>,
    pub pageSize: Option<i64>,
    pub editorPick: Option<i32>,
    pub minimalReputation: Option<i64>,
    pub maximalReputation: Option<i64>,
    pub excludeEditorPickChannel: Option<i32>,
    pub tag: Option<String>,
    pub subStatus: Option<i32>,
    pub isTag: Option<i32>,
    pub sub_name: Option<String>,
    pub sub_url: Option<String>,
    pub sort: Option<String>,
    pub direction: Option<String>
}
