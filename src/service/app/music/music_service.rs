use diesel::dsl::any;
use rocket::serde::json::Json;
use rust_wheel::common::query::pagination::{Paginated, PaginateForQueryFragment, PaginateForQuerySource};
use rust_wheel::common::util::model_convert::map_pagination_res;
use rust_wheel::config::db::config;
use rust_wheel::model::response::pagination_response::PaginationResponse;
use crate::models::{Favorites, Music};
use crate::diesel::prelude::*;
use crate::model::request::app::music::fav::fav_music_request::FavMusicRequest;

pub fn get_music_by_sourceIds<T>(sourceIds: Vec<String>) -> Vec<Music> {
    use crate::model::diesel::rhythm::rhythm_schema::songs::dsl::*;
    let connection = config::establish_music_connection();
    let result = songs.filter(source_id.eq(any(sourceIds)))
        .load::<Music>(&connection)
        .expect("search songs failed");
    return result;
}





