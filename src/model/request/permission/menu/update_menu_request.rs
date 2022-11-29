use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
#[allow(non_snake_case)]
pub struct UpdateMenuRequest {
    pub id: i32,
    pub sort: i32,
    pub path: String,
    pub component: Option<String>
}
