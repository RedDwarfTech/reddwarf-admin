use diesel::ExpressionMethods;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::query::pagination_fragment::PaginateForQueryFragment;
use rust_wheel::common::util::collection_util::take;
use rust_wheel::common::util::model_convert::map_pagination_res;
use rust_wheel::common::util::security_util::get_sha;
use rust_wheel::common::util::time_util::get_current_millisecond;
use rust_wheel::common::wrapper::rocket_http_resp::{box_error_rest_response, box_rest_response};
use rust_wheel::config::db::config;
use rust_wheel::model::response::pagination_response::PaginationResponse;
use rust_wheel::model::user::login_user_info::LoginUserInfo;
use crate::common::db::database::get_conn;
use crate::diesel::prelude::*;
use crate::model::diesel::dolphin::custom_dolphin_models::{AdminUserAdd, UserRoleAdd};
use crate::model::diesel::dolphin::dolphin_models::{
    AdminUser, MenuResource, RolePermission, UserRole,
};
use crate::model::request::user::add_user_request::AddUserRequest;
use crate::model::request::user::admin_pwd_edit_request::AdminPwdEditRequest;
use crate::model::request::user::user_request::UserRequest;
use crate::model::request::user::user_role_request::UserRoleRequest;
use crate::model::response::permission::menu::dynamic_menu_response::DynamicMenuResponse;

pub fn admin_user_query<T>(request: UserRequest) -> PaginationResponse<Vec<AdminUser>> {
    use crate::model::diesel::dolphin::dolphin_schema::admin_users as admin_user_table;
    let mut query = admin_user_table::table.into_boxed::<diesel::pg::Pg>();
    if let Some(filter_user_name) = request.user_name {
        query = query.filter(admin_user_table::user_name.eq(filter_user_name));
    }
    let query = query
        .paginate(request.pageNum, false)
        .per_page(request.pageSize);
    let query_result: QueryResult<(Vec<_>, i64, i64)> =
        query.load_and_count_pages_total::<AdminUser>(&mut get_conn());
    let page_result = map_pagination_res(query_result, request.pageNum, request.pageSize);
    return page_result;
}

pub fn role_menu_list(filter_role_id: i32) -> Vec<MenuResource> {
    
    use crate::model::diesel::dolphin::dolphin_schema::role_permission as role_permission_schema;
    use crate::model::diesel::dolphin::dolphin_schema::role_permission::dsl::*;
    let role_permissions = role_permission
        .filter(role_permission_schema::dsl::role_id.eq(filter_role_id))
        .load::<RolePermission>(&mut get_conn())
        .expect("load role permission failed");
    if role_permissions.is_empty() {
        return Vec::new();
    }
    // get user menus
    let permission_ids: Vec<i32> = role_permissions
        .iter()
        .map(|item| item.permission_id)
        .collect();
    use crate::model::diesel::dolphin::dolphin_schema::menu_resource as menu_resource_schema;
    use crate::model::diesel::dolphin::dolphin_schema::menu_resource::dsl::*;
    let menu_predicate = menu_resource_schema::dsl::id
        .eq_any(permission_ids)
        .and(menu_resource_schema::dsl::parent_id.ne(0));
    let menus = menu_resource
        .filter(menu_predicate)
        .order(sort.asc())
        .load::<MenuResource>(&mut get_conn())
        .expect("load menus failed");
    if menus.is_empty() {
        return Vec::new();
    }
    return menus;
}

pub fn admin_user_menus_list(login_user_info: LoginUserInfo) -> Vec<MenuResource> {
    use crate::model::diesel::dolphin::dolphin_schema::user_role::dsl::*;
    // get user roles
    let roles = user_role
        .filter(user_id.eq(login_user_info.userId))
        .load::<UserRole>(&mut get_conn())
        .expect("load user role failed");
    if roles.is_empty() {
        return Vec::new();
    }
    // get roles permission
    let role_ids: Vec<i32> = roles.iter().map(|item| item.role_id).collect();
    use crate::model::diesel::dolphin::dolphin_schema::role_permission as role_permission_schema;
    use crate::model::diesel::dolphin::dolphin_schema::role_permission::dsl::*;
    let role_permissions = role_permission
        .filter(role_permission_schema::dsl::role_id.eq_any(role_ids))
        .load::<RolePermission>(&mut get_conn())
        .expect("load role permission failed");
    if role_permissions.is_empty() {
        return Vec::new();
    }
    // get user menus
    let permission_ids: Vec<i32> = role_permissions
        .iter()
        .map(|item| item.permission_id)
        .collect();
    use crate::model::diesel::dolphin::dolphin_schema::menu_resource as menu_resource_schema;
    use crate::model::diesel::dolphin::dolphin_schema::menu_resource::dsl::*;
    let menu_predicate = menu_resource_schema::dsl::id.eq_any(permission_ids)
        .and(menu_resource_schema::dsl::parent_id.ne(0));
    let menus = menu_resource
        .filter(menu_predicate)
        .order(sort.asc())
        .load::<MenuResource>(&mut get_conn())
        .expect("load menus failed");
    if menus.is_empty() {
        return Vec::new();
    }
    return menus;
}

pub fn user_roles(filter_user_id: i64) -> Vec<UserRole> {
    use crate::model::diesel::dolphin::dolphin_schema::user_role::dsl::*;
    let user_roles = user_role
        .filter(user_id.eq(filter_user_id))
        .load::<UserRole>(&mut get_conn())
        .expect("get user role failed");
    return user_roles;
}

pub fn add_admin_user(request: Json<AddUserRequest>) -> content::RawJson<String> {
    use crate::model::diesel::dolphin::dolphin_schema::admin_users::dsl::*;
    
    let admin_users_result = admin_users
        .filter(user_name.eq(request.userName.to_string()))
        .load::<AdminUser>(&mut get_conn())
        .expect("get user role failed");
    if admin_users_result.len() > 0 {
        return box_error_rest_response(
            "user exists",
            "ADMIN_USER_EXISTS".parse().unwrap(),
            "user exists".parse().unwrap(),
        );
    }
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .map(char::from)
        .take(10)
        .collect();
    let sha_password = get_sha(String::from("123456"), "123456");
    let new_admin_user = AdminUserAdd {
        nickname: rand_string,
        avatar_url: None,
        phone: request.phone.to_string(),
        updated_time: get_current_millisecond(),
        created_time: get_current_millisecond(),
        salt: "123456".to_string(),
        pwd: sha_password,
        sex: None,
        level_type: None,
        phone_region: None,
        country_code: None,
        user_status: 1,
        user_name: request.userName.to_string(),
        org_id: request.orgId,
    };
    diesel::insert_into(admin_users)
        .values(&new_admin_user)
        .execute(&mut get_conn())
        .unwrap();
    return box_rest_response("ok");
}

pub fn save_user_roles_impl(request: Json<UserRoleRequest>) -> content::RawJson<String> {
    use crate::model::diesel::dolphin::dolphin_schema::user_role::dsl::*;
    let mut connection = config::establish_connection();
    let transaction_result = connection
        .build_transaction()
        .repeatable_read()
        .run::<_, diesel::result::Error, _>(|_| {
            let delete_result = diesel::delete(user_role.filter(user_id.eq(request.userId)))
                .execute(&mut get_conn());
            match delete_result {
                Ok(_v) => {
                    let mut user_role_rec = Vec::new();
                    let current_time = get_current_millisecond();
                    for new_role_id in &request.roleIds {
                        let rec = UserRoleAdd {
                            user_id: request.userId,
                            role_id: *new_role_id,
                            created_time: current_time,
                            updated_time: current_time,
                        };
                        user_role_rec.push(rec);
                    }
                    diesel::insert_into(user_role)
                        .values(&user_role_rec)
                        .execute(&mut get_conn())
                        .unwrap();
                    Ok(())
                }
                Err(e) => {
                    error!("delete user role error:{}", e.to_string());
                    Ok(())
                }
            }
        });
    match transaction_result {
        Ok(_v) => {}
        Err(e) => {
            error!("error:{}", e.to_string());
        }
    }
    return box_rest_response("ok");
}

pub fn admin_user_menus(login_user_info: LoginUserInfo) -> Vec<DynamicMenuResponse> {
    let menus = admin_user_menus_list(login_user_info);
    let root_menus = get_root_menus(&menus);
    return convert_menu_to_tree(&root_menus, &menus);
}

pub fn get_root_menus(menus: &Vec<MenuResource>) -> Vec<MenuResource> {
    let mut root_menus = Vec::new();
    for menu in menus {
        let split_path: Vec<String> = menu
            .tree_id_path
            .split("-")
            .map(|s| s.to_string())
            .collect();
        if split_path.len() == 2 {
            root_menus.push(menu.clone());
        }
    }
    return root_menus;
}

pub fn convert_menu_to_tree(
    root_menus: &Vec<MenuResource>,
    sub_menus: &Vec<MenuResource>,
) -> Vec<DynamicMenuResponse> {
    let mut menu_res_list = Vec::new();
    for root_menu in root_menus {
        let mut origin_menu_res_list = Vec::new();
        let mut menu_res = DynamicMenuResponse::from(root_menu);
        for sub_menu in sub_menus {
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

pub fn admin_password_edit(
    request: &Json<AdminPwdEditRequest>,
    login_user_info: LoginUserInfo,
) -> content::RawJson<String> {
    use crate::model::diesel::dolphin::dolphin_schema::admin_users::dsl::*;
    
    // verify legacy password
    let predicate =
        crate::model::diesel::dolphin::dolphin_schema::admin_users::id.eq(login_user_info.userId);
    let db_admin_user = admin_users
        .filter(&predicate)
        .limit(1)
        .load::<AdminUser>(&mut get_conn())
        .expect("query admin user failed");
    let single_user = take(db_admin_user, 0).unwrap();
    let pwd_salt = single_user.salt;
    let sha_password = get_sha(String::from(&request.oldPassword), &pwd_salt);
    if sha_password.eq(&single_user.pwd.as_str()) {
        let new_password = get_sha(String::from(&request.newPassword), &pwd_salt);
        diesel::update(admin_users.filter(predicate))
            .set(pwd.eq(new_password))
            .get_result::<AdminUser>(&mut get_conn())
            .expect("unable to update new password");
    } else {
        return box_error_rest_response(
            "",
            "00100100064007".parse().unwrap(),
            "old password did not match".parse().unwrap(),
        );
    }
    return box_rest_response("ok");
}
