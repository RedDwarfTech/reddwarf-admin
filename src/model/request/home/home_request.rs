use rocket::serde::Deserialize;
use rocket::serde::Serialize;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct HomeRequest {
    pub word: String,
    pub from: String,
    pub to: String
}