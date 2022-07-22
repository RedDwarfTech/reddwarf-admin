
#[cfg(test)]
mod test {
    use rocket::serde::json::Json;
    use rust_wheel::common::util::time_util::end_of_today;

    use crate::model::request::user::password_request::PasswordRequest;
    use crate::service::user::user_service::password_edit;

    #[test]
    fn end_of_day_test(){
        let end_of_day = end_of_today();
        println!("{}",end_of_day)
    }
}




