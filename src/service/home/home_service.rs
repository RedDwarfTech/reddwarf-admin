use rocket::serde::json::Json;
use rust_wheel::common::query::pagination::{Paginated, PaginateForQueryFragment, PaginateForQuerySource};
use rust_wheel::config::db::config;
use crate::models::{Favorites};
use crate::diesel::prelude::*;
use crate::model::request::app::music::fav::fav_music_request::FavMusicRequest;

pub fn fav_music_query<T>(request: Json<FavMusicRequest>) -> Paginated<Vec<Favorites>> {
    use crate::model::diesel::rhythm::rhythm_schema::favorites::dsl::*;
    let connection = config::establish_music_connection();
    let query = favorites.filter(like_status.eq(1)).paginate(request.pageNum).per_page(request.pageSize);
    let query_result = query.load_and_count_pages::<Favorites>(&connection).unwrap();
    let page_result = Paginated{
        query: query_result.0,
        page: request.pageNum,
        per_page: request.pageSize,
        is_sub_query: false
    };
    return page_result;
}


