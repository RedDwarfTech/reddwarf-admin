
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
        use crate::model::diesel::rhythm::rhythm_schema::favorites::dsl::*;
        use rust_wheel::common::query::pagination::{PaginateForQueryFragment, PaginateForQuerySource};

        let dashboards = fav_music_query::<Favorites>();
        let res = ApiResponse {
            result: "dashboards",
            ..Default::default()
        };
        let response_json = serde_json::to_string(&res).unwrap();
        println!("{:?}", 1);
    }
}

