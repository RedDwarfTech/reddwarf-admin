
#[cfg(test)]
mod test {

    use std::env;
    use diesel::{Connection, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
    use rust_wheel::common::query::pagination::PaginateForQuerySource;
    use rust_wheel::config::db::config::establish_music_connection;
    use crate::model::diesel::rhythm::rhythm_schema::favorites::dsl::favorites;
    use crate::model::diesel::rhythm::rhythm_schema::favorites::like_status;
    use crate::models::Favorites;
    use crate::service::home::home_service::fav_music_query;

    #[test]
    fn page_test(){
        use crate::model::diesel::rhythm::rhythm_schema::favorites::dsl::*;
        use rust_wheel::common::query::pagination::{PaginateForQueryFragment, PaginateForQuerySource};

        fav_music_query();
        //let conn = establish_music_connection();
        //let query = favorites.filter(like_status.eq(1)).paginate(1).per_page(10);
        //let query_result = query.load_and_count_pages::<Favorites>(&conn).unwrap();
        println!("{:?}", 1);
    }
}

