use diesel::sql_query;
use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::query::pagination::PaginateForQueryFragment;
use rust_wheel::common::util::collection_util::take;
use rust_wheel::common::util::model_convert::{box_error_rest_response, box_rest_response, map_pagination_from_list};
use rust_wheel::common::util::security_util::get_sha;
use rust_wheel::config::db::config;
use rust_wheel::model::response::pagination::Pagination;
use rust_wheel::model::response::pagination_response::PaginationResponse;

use crate::diesel::prelude::*;
use crate::model::diesel::dolphin::dolphin_models::AdminUser;
use crate::model::diesel::dolphin::dolphin_models::MenuResource;
use crate::model::request::permission::menu::menu_request::MenuRequest;
use crate::model::request::user::password_request::PasswordRequest;
use crate::model::response::permission::menu::menu_response::MenuResponse;

/**
 * find all sub menu with PostgreSQL CTE(Common Table Expressions)
 * 
 * 
 * 
 */
pub fn org_query_full_tree<T>(filter_parent_id: i32) -> Vec<MenuResponse>{
    use crate::model::diesel::dolphin::dolphin_schema::menu_resource::dsl::*;
    let connection = config::establish_connection();
    let predicate = crate::model::diesel::dolphin::dolphin_schema::menu_resource::parent_id.eq(filter_parent_id);
    let root_menus = menu_resource.filter(&predicate)
        .order(sort.asc())
        .load::<MenuResource>(&connection)
        .expect("Error find menu resource");
    return find_sub_org_cte_impl(&root_menus);
}

pub fn find_sub_org_cte_impl(root_menus: &Vec<MenuResource>) -> Vec<MenuResponse>{
    let connection = config::establish_connection();
    let cte_query_sub_menus = " with recursive sub_menus as
    (
      SELECT
        id,
        name,
        name_zh,
        res_type,
        created_time,
        updated_time,
        remark,
        path,
        parent_id,
        sort,
        component,
        tree_id_path
      FROM menu_resource mr
      WHERE id = 5
      UNION ALL
      SELECT
        origin.id,
        origin.name,
        origin.name_zh,
        origin.res_type,
        origin.created_time,
        origin.updated_time,
        origin.remark,
        origin.path,
        origin.parent_id,
        origin.sort,
        origin.component,
        origin.tree_id_path
      FROM sub_menus
      JOIN menu_resource origin
      ON origin.parent_id = sub_menus.id
    )
    SELECT
        id,
        name,
        name_zh,
        res_type,
        created_time,
        updated_time,
        remark,
        path,
        parent_id,
        sort,
        component,
        tree_id_path
    FROM sub_menus
    ORDER BY sort ASC;
    ";
    let cte_menus = sql_query(cte_query_sub_menus)
        .load::<MenuResource>(&connection)
        .expect("Error find menu resource");
    return convert_menu_to_tree(root_menus, &cte_menus);
}

/**
** convert the list menu to tree recursive
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
pub fn org_query_tree<T>(request: &Json<MenuRequest>) -> PaginationResponse<Vec<MenuResponse>> {
    use crate::model::diesel::dolphin::dolphin_schema::menu_resource::dsl::*;
    let connection = config::establish_connection();
    let predicate = crate::model::diesel::dolphin::dolphin_schema::menu_resource::parent_id.eq(request.parentId);
    let query = menu_resource.filter(&predicate)
        .paginate(request.pageNum,false)
        .per_page(request.pageSize);
    let query_result: QueryResult<(Vec<_>, i64, i64)> = query.load_and_count_pages_total::<MenuResource>(&connection);
    let menu_responses = find_sub_org_cte_impl(&query_result.as_ref().unwrap().0);
    let total = query_result.as_ref().unwrap().2;
    let page_result = map_pagination_from_list( menu_responses,request.pageNum, request.pageSize,total);
    return page_result;
}

pub fn org_edit(request: &Json<PasswordRequest>) -> content::Json<String> {
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