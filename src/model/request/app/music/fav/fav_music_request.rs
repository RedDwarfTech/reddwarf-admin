use rocket::serde::Deserialize;
use rocket::serde::Serialize;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct FavMusicRequest {
    pub userId: i64,
    pub pageNum: i64,
    pub pageSize: i64
}