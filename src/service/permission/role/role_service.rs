use crate::model::request::permission::role::role_menu_bind_request::RoleMenuBindRequest;
use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::query::pagination::PaginateForQueryFragment;
use rust_wheel::common::util::collection_util::take;
use rust_wheel::common::util::model_convert::{box_error_rest_response, box_rest_response, map_pagination_res};
use rust_wheel::common::util::security_util::get_sha;
use rust_wheel::common::util::time_util::get_current_millisecond;
use rust_wheel::config::db::config;
use rust_wheel::model::response::pagination_response::PaginationResponse;

use crate::diesel::prelude::*;
use crate::model::diesel::dolphin::custom_dolphin_models::RolePermissionAdd;
use crate::model::diesel::dolphin::dolphin_models::{AdminUser, Role, RolePermission};
use crate::model::request::permission::role::role_request::RoleRequest;
use crate::model::request::user::password_request::PasswordRequest;

pub fn role_query<T>(request: &Json<RoleRequest>) -> PaginationResponse<Vec<Role>> {
    use crate::model::diesel::dolphin::dolphin_schema::role::dsl::*;
    let connection = config::establish_connection();
    let query = role.filter(id.gt(0))
        .paginate(request.pageNum,false)
        .per_page(request.pageSize);
    let query_result: QueryResult<(Vec<_>, i64, i64)> = query.load_and_count_pages_total::<Role>(&connection);
    let page_result = map_pagination_res(query_result, request.pageNum, request.pageSize);
    return page_result;
}

pub fn edit_role_menu(request: &Json<RoleMenuBindRequest>) -> content::Json<String> {
    // delete the legacy record
    use crate::model::diesel::dolphin::dolphin_schema::role_permission::dsl::*;
    let connection = config::establish_connection();
    diesel::delete(role_permission.filter(role_id.eq(request.roleId))).execute(&connection);
    // add the new permission record
    let mut role_permission_rec = Vec::new();
    let current_time = get_current_millisecond();
    for menu_id in &request.menuIds {
        let rec = RolePermissionAdd{
            role_id: Option::from(request.roleId),
            permission_id: Option::from(*menu_id),
            created_time: Option::from(current_time),
            updated_time: Option::from(current_time)
        };
        role_permission_rec.push(rec);
    }
    diesel::insert_into(role_permission)
        .values(&role_permission_rec)
        .execute(&connection)
        .unwrap();
    return box_rest_response("ok");
}

pub fn role_edit(request: &Json<PasswordRequest>) -> content::Json<String> {
    use crate::model::diesel::dolphin::dolphin_schema::admin_users::dsl::*;
    let connection = config::establish_connection();
    // verify legacy password
    let request_user_name:String = String::from(&request.userName);
    let predicate = crate::model::diesel::dolphin::dolphin_schema::admin_users::phone.eq(request_user_name);
    let db_admin_user = admin_users.filter(&predicate)
        .limit(1)
        .load::<AdminUser>(&connection)
        .expect("query admin user failed");
    let single_user = take(db_admin_user,0).unwrap();
    let pwd_salt = single_user.salt.unwrap();
    let sha_password = get_sha(String::from(&request.oldPassword), &pwd_salt);
    if sha_password.eq(&single_user.pwd.unwrap().as_str()){
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