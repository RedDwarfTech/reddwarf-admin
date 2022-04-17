use diesel::sql_query;
use rust_wheel::model::response::pagination::Pagination;
use crate::model::request::permission::menu::menu_request::MenuRequest;
use crate::model::diesel::dolphin::dolphin_models::MenuResource;
use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::query::pagination::PaginateForQueryFragment;
use rust_wheel::common::util::collection_util::take;
use rust_wheel::common::util::model_convert::{box_error_rest_response, box_rest_response};
use rust_wheel::common::util::security_util::get_sha;
use rust_wheel::config::db::config;
use rust_wheel::model::response::pagination_response::PaginationResponse;

use crate::diesel::prelude::*;
use crate::model::diesel::dolphin::dolphin_models::{AdminUser};
use crate::model::request::user::password_request::PasswordRequest;
use crate::model::response::permission::menu::menu_response::MenuResponse;

/**
 * find all sub menu with PostgreSQL CTE(Common Table Expressions)
 * 
 * 
 * 
 */
pub fn menu_query_full_tree<T>(request: &Json<MenuRequest>) -> Vec<MenuResponse>{
    use crate::model::diesel::dolphin::dolphin_schema::menu_resource::dsl::*;
    let connection = config::establish_connection();
    let predicate = crate::model::diesel::dolphin::dolphin_schema::menu_resource::parent_id.eq(request.parentId);
    let root_menus = menu_resource.filter(&predicate)
        .load::<MenuResource>(&connection)
        .expect("Error find menu resource");
    return find_sub_menu_cte_impl(&root_menus);
}

pub fn find_sub_menu_cte_impl(root_menus: &Vec<MenuResource>) -> Vec<MenuResponse>{
    let connection = config::establish_connection();
    let cte_query_sub_menus = " with recursive sub_menus as
    (
      SELECT
        id,
        name,
        res_type,
        created_time,
        updated_time,
        remark,
        path,
        parent_id
      FROM menu_resource mr
      WHERE id = 0
      UNION ALL
      SELECT
        origin.id,
        origin.name,
        origin.res_type,
        origin.created_time,
        origin.updated_time,
        origin.remark,
        origin.path,
        origin.parent_id
      FROM sub_menus
      JOIN menu_resource origin
      ON origin.parent_id = sub_menus.id
    )
    SELECT
        id,
        name,
        res_type,
        created_time,
        updated_time,
        remark,
        path,
        parent_id
    FROM sub_menus;
    ";
    let cte_menus = sql_query(cte_query_sub_menus)
        .load::<MenuResource>(&connection)
        .expect("Error find menu resource");
    return convert_menu_to_tree(root_menus, &cte_menus);
}

/**
* convert the list menu to tree recursive
**/
pub fn convert_menu_to_tree(root_menus: &Vec<MenuResource>, sub_menus: &Vec<MenuResource>) -> Vec<MenuResponse>{
    let mut menu_res_list = Vec::new();
    for root_menu in root_menus {
        let mut origin_menu_res_list = Vec::new();
        let mut menu_res = MenuResponse::from(root_menu);
        for sub_menu in sub_menus{
            if sub_menu.parent_id == root_menu.id {
                let menu_res_sub = MenuResponse::from(sub_menu);
                menu_res.children.push(menu_res_sub);
                origin_menu_res_list.push(sub_menu.clone());
            }
        }
        if !menu_res.children.is_empty() {
            menu_res.children = convert_menu_to_tree(&origin_menu_res_list, sub_menus);
        }
        menu_res_list.push(menu_res);
    }
    return menu_res_list;
}

/**
 * this function query the menu tree for showing in table
 * for the performance issue
 * only query 2 level from current parent level
 */
pub fn menu_query_tree<T>(request: &Json<MenuRequest>) -> PaginationResponse<Vec<MenuResponse>> {
    use crate::model::diesel::dolphin::dolphin_schema::menu_resource::dsl::*;
    let connection = config::establish_connection();
    let predicate = crate::model::diesel::dolphin::dolphin_schema::menu_resource::parent_id.eq(request.parentId);
    let query = menu_resource.filter(&predicate)
        .paginate(request.pageNum,false)
        .per_page(request.pageSize);
    let query_result: QueryResult<(Vec<_>, i64, i64)> = query.load_and_count_pages_total::<MenuResource>(&connection);
    let menu_responses = find_sub_menu(&query_result.as_ref().unwrap().0);
    let total = query_result.as_ref().unwrap().2;
    let page_result = map_pagination_res_local(total, request.pageNum, request.pageSize,menu_responses);
    return page_result;
}

/**
 * for the performance issue
 * find the 2 level of tree 
 */
pub fn find_sub_menu(pmenus: &Vec<MenuResource>) -> Vec<MenuResponse>{
    use diesel::pg::expression::dsl::any;
    use crate::model::diesel::dolphin::dolphin_schema::menu_resource::dsl::*;
    let connection = config::establish_connection();
    let parent_ids: Vec<i32> = pmenus
        .iter()
        .map(|item| item.id)
        .collect();
    let predicate = crate::model::diesel::dolphin::dolphin_schema::menu_resource::parent_id.eq(any(parent_ids));
    let menus = menu_resource.filter(&predicate)
    .load::<MenuResource>(&connection)
    .expect("Error find menu resource");
    return convert_menu_to_tree(pmenus,&menus);
}

pub fn map_pagination_res_local<U>(total: i64, page_num: i64,page_size: i64, data: Vec<U>) -> PaginationResponse<Vec<U>>{
    let page_result = Pagination{
        pageNum: page_num,
        pageSize: page_size,
        total: total
    };
    let resp = PaginationResponse{
        pagination: page_result,
        list: data
    };
    return resp;
}

pub fn menu_edit(request: &Json<PasswordRequest>) -> content::Json<String> {
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