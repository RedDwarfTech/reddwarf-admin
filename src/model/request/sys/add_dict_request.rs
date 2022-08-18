use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
#[allow(non_snake_case)]
pub struct AddDictRequest {
    pub key: i32,
    pub dict_type: String,
    pub value: String,
    pub show_value: String,
    pub dict_type_name: String,
}