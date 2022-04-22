use diesel::dsl::any;
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
use crate::model::diesel::dolphin::dolphin_models::{AdminUser, MenuResource, RolePermission, User, UserRole};
use crate::model::diesel::dolphin::dolphin_schema::article_favorites::user_id;
use crate::model::request::user::password_request::PasswordRequest;
use crate::model::request::user::user_request::UserRequest;
use crate::model::response::permission::menu::dynamic_menu_response::DynamicMenuResponse;

pub fn admin_user_query<T>(request: &Json<UserRequest>) -> PaginationResponse<Vec<AdminUser>> {
    use crate::model::diesel::dolphin::dolphin_schema::admin_users::dsl::*;
    let connection = config::establish_connection();
    let query = admin_users.filter(id.gt(0))
        .paginate(request.pageNum,false)
        .per_page(request.pageSize);
    let query_result: QueryResult<(Vec<_>, i64, i64)> = query.load_and_count_pages_total::<AdminUser>(&connection);
    let page_result = map_pagination_res(query_result, request.pageNum, request.pageSize);
    return page_result;
}

pub fn admin_user_menus_list(login_user_info: LoginUserInfo) -> Vec<MenuResource> {
    use crate::model::diesel::dolphin::dolphin_schema::user_role::dsl::*;
    let connection = config::establish_connection();
    // get user roles
    let roles = user_role.filter(user_id.eq(login_user_info.userId))
        .load::<UserRole>(&connection)
        .expect("load user role failed");
    if roles.is_empty(){
        return Vec::new();
    }
    // get roles permission
    let role_ids: Vec<i32> = roles
        .iter()
        .map(|item| item.role_id)
        .collect();
    use crate::model::diesel::dolphin::dolphin_schema::role_permission::dsl::*;
    use crate::model::diesel::dolphin::dolphin_schema::role_permission as role_permission_schema;
    let role_permissions = role_permission.filter(role_permission_schema::dsl::role_id.eq(any(role_ids)))
        .load::<RolePermission>(&connection)
        .expect("load role permission failed");
    if role_permissions.is_empty() {
        return Vec::new();
    }
    // get user menus
    let permission_ids: Vec<i32> = role_permissions
        .iter()
        .map(|item|item.permission_id)
        .collect();
    use crate::model::diesel::dolphin::dolphin_schema::menu_resource::dsl::*;
    use crate::model::diesel::dolphin::dolphin_schema::menu_resource as menu_resource_schema;
    let menus = menu_resource.filter(menu_resource_schema::dsl::id.eq(any(permission_ids)))
        .order(sort.asc())
        .load::<MenuResource>(&connection)
        .expect("load menus failed");
    if menus.is_empty() {
        return Vec::new();
    }
    return menus;
}

pub fn user_roles(login_user_info: LoginUserInfo) -> Vec<UserRole>{
    use crate::model::diesel::dolphin::dolphin_schema::user_role::dsl::*;
    let connection = config::establish_connection();
    let user_roles = user_role.filter(user_id.eq(login_user_info.userId))
        .load::<UserRole>(&connection)
        .expect("get user role failed");
    return user_roles;
}

pub fn admin_user_menus(login_user_info: LoginUserInfo) -> Vec<DynamicMenuResponse> {
    let menus = admin_user_menus_list(login_user_info);
    let root_menus = get_root_menus(&menus);
    return convert_menu_to_tree(&root_menus,&menus);
}

pub fn get_root_menus(menus: &Vec<MenuResource>) -> Vec<MenuResource>{
    let mut root_menus = Vec::new();
    let ids:Vec<i32> = menus.iter().map(|item| item.id).collect();
    for menu in menus {
        if !ids.contains(&menu.parent_id){
            root_menus.push(menu.clone());
        }
    }
    return root_menus;
}

pub fn convert_menu_to_tree(root_menus: &Vec<MenuResource>, sub_menus: &Vec<MenuResource>) -> Vec<DynamicMenuResponse>{
    let mut menu_res_list = Vec::new();
    for root_menu in root_menus {
        let mut origin_menu_res_list = Vec::new();
        let mut menu_res = DynamicMenuResponse::from(root_menu);
        for sub_menu in sub_menus{
            if sub_menu.parent_id == root_menu.id {
                let menu_res_sub = DynamicMenuResponse::from(sub_menu);
                menu_res.routes.push(menu_res_sub);
                origin_menu_res_list.push(sub_menu.clone());
            }
        }
        if !menu_res.routes.is_empty() {
            menu_res.routes = convert_menu_to_tree(&origin_menu_res_list, sub_menus);
        }
        menu_res_list.push(menu_res);
    }
    return menu_res_list;
}

pub fn admin_password_edit(request: &Json<PasswordRequest>) -> content::Json<String> {
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