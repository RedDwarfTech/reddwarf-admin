use diesel::sql_query;
use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::query::pagination::PaginateForQueryFragment;
use rust_wheel::common::util::collection_util::take;
use rust_wheel::common::util::model_convert::{box_error_rest_response, box_rest_response, map_pagination_from_list};
use rust_wheel::common::util::security_util::get_sha;
use rust_wheel::config::db::config;
use rust_wheel::model::response::pagination_response::PaginationResponse;

use crate::diesel::prelude::*;
use crate::model::diesel::dolphin::dolphin_models::{AdminUser, Org};
use crate::model::request::permission::org::org_request::OrgRequest;
use crate::model::request::user::password_request::PasswordRequest;
use crate::model::response::permission::org::org_response::OrgResponse;

/**
 * find all sub menu with PostgreSQL CTE(Common Table Expressions)
 * 
 * 
 * 
 */
pub fn org_query_full_tree<T>(filter_parent_id: i32) -> Vec<OrgResponse>{
    use crate::model::diesel::dolphin::dolphin_schema::org::dsl::*;
    let connection = config::establish_connection();
    let predicate = crate::model::diesel::dolphin::dolphin_schema::org::parent_id.eq(filter_parent_id);
    let root_menus = org.filter(&predicate)
        .order(sort.asc())
        .load::<Org>(&connection)
        .expect("Error find menu resource");
    return find_sub_org_cte_impl(&root_menus);
}

pub fn find_sub_org_cte_impl(root_menus: &Vec<Org>) -> Vec<OrgResponse>{
    let connection = config::establish_connection();
    let cte_query_sub_menus = " with recursive sub_org as
    (
      SELECT
        id,
        org_name,
        created_time,
        updated_time,
        parent_id,
        sort,
        tree_id_path
      FROM org mr
      WHERE id = 5
      UNION ALL
      SELECT
        origin.id,
        origin.org_name,
        origin.created_time,
        origin.updated_time,
        origin.parent_id,
        origin.sort,
        origin.tree_id_path
      FROM org
      JOIN org origin
      ON origin.parent_id = org.id
    )
    SELECT
        id,
        org_name,
        created_time,
        updated_time,
        parent_id,
        sort,
        tree_id_path
    FROM sub_org
    ORDER BY sort ASC;
    ";
    let cte_menus = sql_query(cte_query_sub_menus)
        .load::<Org>(&connection)
        .expect("Error find menu resource");
    return convert_org_to_tree(root_menus, &cte_menus);
}

/**
** convert the list menu to tree recursive
**/
pub fn convert_org_to_tree(root_menus: &Vec<Org>, sub_menus: &Vec<Org>) -> Vec<OrgResponse>{
    let mut menu_res_list = Vec::new();
    for root_menu in root_menus {
        let mut origin_menu_res_list = Vec::new();
        let mut menu_res = OrgResponse::from(root_menu);
        for sub_menu in sub_menus{
            if sub_menu.parent_id == root_menu.id {
                let menu_res_sub = OrgResponse::from(sub_menu);
                menu_res.children.push(menu_res_sub);
                origin_menu_res_list.push(sub_menu.clone());
            }
        }
        if !menu_res.children.is_empty() {
            menu_res.children = convert_org_to_tree(&origin_menu_res_list, sub_menus);
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
pub fn org_query_tree<T>(request: &Json<OrgRequest>) -> PaginationResponse<Vec<OrgResponse>> {
    use crate::model::diesel::dolphin::dolphin_schema::org::dsl::*;
    let connection = config::establish_connection();
    let predicate = crate::model::diesel::dolphin::dolphin_schema::org::parent_id.eq(request.parentId);
    let query = org.filter(&predicate)
        .paginate(request.pageNum,false)
        .per_page(request.pageSize);
    let query_result: QueryResult<(Vec<_>, i64, i64)> = query.load_and_count_pages_total::<Org>(&connection);
    let menu_responses = find_sub_org_cte_impl(&query_result.as_ref().unwrap().0);
    let total = query_result.as_ref().unwrap().2;
    let page_result = map_pagination_from_list( menu_responses,request.pageNum, request.pageSize,total);
    return page_result;
}

pub fn org_edit(request: &Json<PasswordRequest>) -> content::RawJson<String> {
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