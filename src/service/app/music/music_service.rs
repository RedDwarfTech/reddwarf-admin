use diesel::dsl::any;
use rust_wheel::config::db::config;

use crate::diesel::prelude::*;
use crate::models::Music;

pub fn get_music_by_source_ids<T>(source_ids: Vec<String>) -> Vec<Music> {
    use crate::model::diesel::rhythm::rhythm_schema::songs::dsl::*;
    let connection = config::establish_music_connection();
    let result = songs.filter(source_id.eq(any(source_ids)))
        .load::<Music>(&connection)
        .expect("search songs failed");
    return result;
}





