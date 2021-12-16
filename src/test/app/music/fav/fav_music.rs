
#[cfg(test)]
mod test {
    use std::env;
    use diesel::{Connection, ExpressionMethods, PgConnection, QueryDsl};
    use rust_wheel::common::query::pagination::PaginateForQuerySource;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn page_test(){
        use crate::schema::favorites::dsl::*;
        use rust_wheel::common::query::pagination::{PaginateForQueryFragment, PaginateForQuerySource};
        let conn = establish_music_connection();
        let query = favorites.filter(like_status.eq(1)).paginate(1).per_page(10);

        println!("{:?}", 1);




    }

    pub fn establish_music_connection() -> PgConnection {
        let database_url = std::env::var("MUSIC_DATABASE_URL").expect("MUSIC_DATABASE_URL must be set");
        PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
    }

    fn verbose() {
        let name = "USER";
        match env::var(name) {
            Ok(v) => println!("{}: {}", name, v),
            Err(e) => panic!("${} is not set ({})", name, e)
        }
    }

    fn short() {
        let v = env::var("MUSIC_DATABASE_URL").expect("$USER is not set");
    }
}

