
#[cfg(test)]
mod test {

    use std::env;
    use diesel::{Connection, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
    use rocket::serde::json::Json;
    use rust_wheel::common::query::pagination::PaginateForQuerySource;
    use rust_wheel::config::db::config::establish_music_connection;
    use rust_wheel::model::response::api_response::ApiResponse;
    use crate::model::diesel::rhythm::rhythm_schema::favorites::dsl::favorites;
    use crate::model::diesel::rhythm::rhythm_schema::favorites::like_status;
    use crate::model::request::user::password_request::PasswordRequest;
    use crate::models::Favorites;
    use crate::service::user::user_service::password_edit;

    #[test]
    fn user_test(){
        let pr = PasswordRequest{
            loginType: 0,
            newPassword: "123456".to_string(),
            oldPassword: "$mycruise123".to_string(),
            userName: "+8615683761628".to_string()
        };
        let jpr = Json(pr);
        let password_request = password_edit(&jpr);
    }
}

