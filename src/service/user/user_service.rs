use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::query::pagination::PaginateForQueryFragment;
use rust_wheel::common::util::collection_util::take;
use rust_wheel::common::util::model_convert::{box_error_rest_response, box_rest_response, map_pagination_res};
use rust_wheel::common::util::security_util::get_sha;
use rust_wheel::config::db::config;
use rust_wheel::model::response::pagination_response::PaginationResponse;
use rust_wheel::model::user::login_user_info::LoginUserInfo;

use crate::diesel::prelude::*;
use crate::model::diesel::dolphin::dolphin_models::{AdminUser, User};
use crate::model::request::user::password_request::PasswordRequest;
use crate::model::request::user::user_request::UserRequest;

pub fn user_query<T>(request: &Json<UserRequest>) -> PaginationResponse<Vec<User>> {
    use crate::model::diesel::dolphin::dolphin_schema::users::dsl::*;
    let connection = config::establish_connection();
    let query = users.filter(id.gt(0))
        .paginate(request.pageNum,false)
        .per_page(request.pageSize);
    let query_result: QueryResult<(Vec<_>, i64, i64)> = query.load_and_count_pages_total::<User>(&connection);
    let page_result = map_pagination_res(query_result, request.pageNum, request.pageSize);
    return page_result;
}

pub fn password_edit(request: &Json<PasswordRequest>,login_user_info: LoginUserInfo) -> content::RawJson<String> {
    use crate::model::diesel::dolphin::dolphin_schema::admin_users::dsl::*;
    let connection = config::establish_connection();
    // verify legacy password
    let predicate = crate::model::diesel::dolphin::dolphin_schema::admin_users::id.eq(login_user_info.userId);
    let db_admin_user = admin_users.filter(&predicate)
        .limit(1)
        .load::<AdminUser>(&connection)
        .expect("query admin user failed");
    let single_user = take(db_admin_user,0).unwrap();
    let pwd_salt = single_user.salt;
    let sha_password = get_sha(String::from(&request.oldPassword), &pwd_salt);
    if sha_password.eq(&single_user.pwd.as_str()){
        let new_password = get_sha(String::from(&request.newPassword),&pwd_salt);
        diesel::update(admin_users.filter(predicate))
            .set(pwd.eq(new_password))
            .get_result::<AdminUser>(&connection)
            .expect("unable to update new password");
    }else{
        return box_error_rest_response("", "00100100064007".parse().unwrap(), "old password did not match".parse().unwrap());
    }
    return box_rest_response("ok");
}