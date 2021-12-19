
#[cfg(test)]
mod test {

    use std::env;
    use diesel::{Connection, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
    use rust_wheel::common::query::pagination::PaginateForQuerySource;
    use rust_wheel::config::db::config::establish_music_connection;
    use rust_wheel::model::response::api_response::ApiResponse;
    use crate::model::diesel::rhythm::rhythm_schema::favorites::dsl::favorites;
    use crate::model::diesel::rhythm::rhythm_schema::favorites::like_status;
    use crate::models::Favorites;
    use crate::service::home::home_service::fav_music_query;

    #[test]
    fn page_test(){

    }
}

