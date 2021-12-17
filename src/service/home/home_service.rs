use rust_wheel::common::query::pagination::{Paginated, PaginateForQueryFragment, PaginateForQuerySource};
use rust_wheel::config::db::config;
use crate::models::{Favorites};
use crate::diesel::prelude::*;

pub fn fav_music_query<T>() -> Paginated<T> {
    use crate::model::diesel::rhythm::rhythm_schema::favorites::dsl::*;
    let connection = config::establish_music_connection();
    let query = favorites.filter(like_status.eq(1)).paginate(1).per_page(10);
    let query_result = query.load_and_count_pages::<Favorites>(&connection).unwrap();
    let page_result = Paginated{
        query: query_result.0,
        page: 1,
        per_page: 10,
        is_sub_query: false
    };

    return page_result;
}


