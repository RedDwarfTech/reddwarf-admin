
#[cfg(test)]
mod test {
    use rocket::serde::json::Json;

    use crate::model::request::user::password_request::PasswordRequest;
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
        //let _password_request = password_edit(&jpr);
    }
}

