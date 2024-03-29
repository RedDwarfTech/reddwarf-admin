use crate::common::db::database::get_conn;
use crate::diesel::prelude::*;
use crate::model::diesel::rhythm::rhythm_models::Song;
use crate::model::request::app::music::music_request::MusicRequest;
use rocket::serde::json::Json;
use rust_wheel::common::query::pagination_fragment::PaginateForQueryFragment;
use rust_wheel::common::util::model_convert::map_pagination_res;
use rust_wheel::model::response::pagination_response::PaginationResponse;
use crate::models::Music;

pub fn music_query<T>(request: &Json<MusicRequest>) -> PaginationResponse<Vec<Song>> {
    use crate::model::diesel::rhythm::rhythm_schema::songs::dsl::*;
    let query = songs
        .filter(id.gt(0))
        .order(created_time.desc())
        .paginate(request.pageNum, false)
        .per_page(request.pageSize);
    let query_result: QueryResult<(Vec<_>, i64, i64)> =
        query.load_and_count_pages_total::<Song>(&mut get_conn());
    let page_result = map_pagination_res(query_result, request.pageNum, request.pageSize);
    return page_result;
}

pub fn get_music_by_source_ids<T>(source_ids: Vec<String>) -> Vec<Music> {
    use crate::model::diesel::rhythm::rhythm_schema::songs::dsl::*;
    let result = songs
        .filter(source_id.eq_any(source_ids))
        .load::<Music>(&mut get_conn())
        .expect("search songs failed");
    return result;
}
