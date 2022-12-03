use diesel::dsl::any;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::map_pagination_res;
use rust_wheel::config::db::config;
use rust_wheel::model::response::pagination_response::PaginationResponse;
use rust_wheel::common::query::pagination::PaginateForQueryFragment;
use crate::diesel::prelude::*;
use crate::model::diesel::rhythm::rhythm_models::Song;
use crate::model::request::app::music::music_request::MusicRequest;
use crate::model::response::app::app_response::AppResponse;
use crate::model::response::app::music::music_response::MusicResponse;
use crate::models::Music;

pub fn music_query<T>(request: &Json<MusicRequest>) -> PaginationResponse<Vec<Song>> {
    use crate::model::diesel::rhythm::rhythm_schema::songs::dsl::*;
    let connection = config::establish_connection();
    let query = songs.filter(id.gt(0))
        .order(created_time.desc())
        .paginate(request.pageNum,false)
        .per_page(request.pageSize);
    let query_result: QueryResult<(Vec<_>, i64, i64)> = query.load_and_count_pages_total::<Song>(&connection);
    let page_result = map_pagination_res(query_result, request.pageNum, request.pageSize);
    return page_result;
}

pub fn get_music_by_source_ids<T>(source_ids: Vec<String>) -> Vec<Music> {
    use crate::model::diesel::rhythm::rhythm_schema::songs::dsl::*;
    let connection = config::establish_music_connection();
    let result = songs.filter(source_id.eq(any(source_ids)))
        .load::<Music>(&connection)
        .expect("search songs failed");
    return result;
}





