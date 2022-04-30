use log::{info, trace, warn};
use rocket::{Request, request};
use rocket::http::Status;
use rocket::log::private::Level;
use rocket::outcome::Outcome;
use rocket::request::FromRequest;

// https://stackoverflow.com/questions/24102325/warning-function-should-have-a-snake-case-identifier-on-by-default
#[allow(non_snake_case)]
pub struct LoginUserInfo {
    pub token: String,
    pub userId: i64,
    pub appId: i32,
    pub xRequestId: String
}

#[derive(Debug)]
pub enum ApiTokenError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for LoginUserInfo {
    type Error = ApiTokenError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token = request.headers().get_one("accessToken");
        let app_id = request.headers().get_one("appId");
        let user_id = request.headers().get_one("userId");
        let x_request_id = request.headers().get_one("X-Request-ID");
        match token {
            Some(token) => {
                let login_user_info = LoginUserInfo {
                    token: token.to_string(),
                    userId: user_id.unwrap().parse::<i64>().unwrap(),
                    appId: app_id.unwrap().parse::<i32>().unwrap(),
                    xRequestId: x_request_id.unwrap().to_string()
                };
                // check validity
                Outcome::Success(login_user_info)
            }
            None => Outcome::Failure((Status::Unauthorized, ApiTokenError::Missing)),
        }
    }
}