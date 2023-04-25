
#[cfg(test)]
mod test {
    use crate::model::request::user::password_request::PasswordRequest;

    #[test]
    fn user_test(){
        let _pr = PasswordRequest{
            loginType: 0,
            newPassword: "123456".to_string(),
            oldPassword: "$mycruise123".to_string(),
            userName: "+8615683761628".to_string()
        };

        //let _password_request = password_edit(&jpr);
    }
}

