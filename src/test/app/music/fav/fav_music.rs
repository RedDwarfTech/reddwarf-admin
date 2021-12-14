
#[cfg(test)]
mod test {
    use rust_wheel::config::db::config::establish_music_connection;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn page_test(){
        use crate::schema::admin_users::dsl::*;
        let conn = establish_music_connection();


    }
}

