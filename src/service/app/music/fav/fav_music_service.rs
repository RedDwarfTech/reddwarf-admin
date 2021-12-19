use rocket::serde::json::Json;
use rust_wheel::common::query::pagination::{PaginateForQueryFragment};
use rust_wheel::common::util::model_convert::map_pagination_res;
use rust_wheel::config::db::config;
use rust_wheel::model::response::pagination_response::PaginationResponse;
use crate::models::{Favorites};
use crate::diesel::prelude::*;
use crate::model::request::app::music::fav::fav_music_request::FavMusicRequest;

pub fn fav_music_query<T>(request: &Json<FavMusicRequest>) -> PaginationResponse<Vec<Favorites>> {
    use crate::model::diesel::rhythm::rhythm_schema::favorites::dsl::*;
    let connection = config::establish_music_connection();
    let query = favorites.filter(like_status.eq(1)).paginate(request.pageNum).per_page(request.pageSize);
    let query_result = query.load_and_count_pages::<Favorites>(&connection);
    let page_result = map_pagination_res(query_result,request.pageNum,request.pageSize);
    return page_result;
}





