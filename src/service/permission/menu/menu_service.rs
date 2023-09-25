use diesel::sql_query;
use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::query::pagination_fragment::PaginateForQueryFragment;
use rust_wheel::common::util::convert_to_tree::convert_to_tree;
use rust_wheel::common::util::model_convert::{map_entity, map_pagination_from_list};
use rust_wheel::common::util::time_util::get_current_millisecond;
use rust_wheel::common::wrapper::rocket_http_resp::box_rest_response;
use rust_wheel::config::db::config;
use rust_wheel::model::response::pagination_response::PaginationResponse;

use crate::common::db::database::{get_conn};
use crate::common::enums::resource_type::ResourceType;
use crate::diesel::prelude::*;
use crate::model::diesel::dolphin::custom_dolphin_models::{MenuResourceAdd, MenuResourcePath};
use crate::model::diesel::dolphin::dolphin_models::MenuResource;
use crate::model::request::permission::menu::add_menu_request::AddMenuRequest;
use crate::model::request::permission::menu::menu_request::MenuRequest;
use crate::model::request::permission::menu::update_menu_request::UpdateMenuRequest;
use crate::model::response::permission::menu::menu_response::MenuResponse;

/**
 * find all sub menu with PostgreSQL CTE(Common Table Expressions)
 * 
 * 
 * 
 */
pub fn menu_query_full_tree<T>(filter_parent_id: i32) -> Vec<MenuResponse>{
    use crate::model::diesel::dolphin::dolphin_schema::menu_resource::dsl::*;
    
    let predicate = crate::model::diesel::dolphin::dolphin_schema::menu_resource::parent_id.eq(filter_parent_id);
    let root_menus = menu_resource.filter(&predicate)
        .order(sort.asc())
        .load::<MenuResource>(&mut get_conn())
        .expect("Error find menu resource");
    return find_sub_menu_cte_impl(&root_menus);
}

pub fn find_sub_menu_cte_impl(_root_menus: &Vec<MenuResource>) -> Vec<MenuResponse>{
    
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
        tree_id_path,
        code
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
        origin.tree_id_path,
        origin.code
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
        tree_id_path,
        code
    FROM sub_menus
    ORDER BY sort ASC;
    ";
    let cte_menus = sql_query(cte_query_sub_menus)
        .load::<MenuResource>(&mut get_conn())
        .expect("Error find menu resource");
    let menu_resource_resp:Vec<MenuResponse> = map_entity(cte_menus);
    return convert_to_tree_impl(&menu_resource_resp);
}

fn convert_to_tree_impl(contents: &Vec<MenuResponse>) -> Vec<MenuResponse> {
    let root_element: Vec<_> = contents.iter()
        .filter(|content| content.parent_id == 0)
        .collect();
    let sub_element: Vec<_> = contents.iter()
        .filter(|content| content.parent_id != 0)
        .collect();
    let result = convert_to_tree(&root_element, &sub_element);
    return result;
}

/**
 * this function query the menu tree for showing in table
 * for the performance issue
 * only query 2 level from current parent level
 */
pub fn menu_query_tree<T>(request: &Json<MenuRequest>) -> PaginationResponse<Vec<MenuResponse>> {
    use crate::model::diesel::dolphin::dolphin_schema::menu_resource::dsl::*;
    let predicate = crate::model::diesel::dolphin::dolphin_schema::menu_resource::parent_id.eq(request.parentId);
    let query = menu_resource.filter(&predicate)
        .paginate(request.pageNum,false)
        .per_page(request.pageSize);
    let query_result: QueryResult<(Vec<_>, i64)> = query.load_and_count_pages::<MenuResource>(&mut get_conn());
    let menu_responses = find_sub_menu_cte_impl(&query_result.as_ref().unwrap().0);
    let total = 200;
    let page_result = map_pagination_from_list(menu_responses,request.pageNum, request.pageSize,total);
    return page_result;
}

pub fn menu_edit(request: &Json<UpdateMenuRequest>) -> content::RawJson<String> {
    use crate::model::diesel::dolphin::dolphin_schema::menu_resource::dsl::*;
    let predicate = id.eq(request.id);
    let update_records = diesel::update(menu_resource.filter(predicate))
        .set((sort.eq(request.sort),path.eq(request.path.to_string()),component.eq(request.component.to_owned())))
        .get_results::<MenuResource>(&mut get_conn())
        .expect("unable to update menu");
    return box_rest_response(update_records.get(0));
}

pub fn menu_add(request: &Json<AddMenuRequest>) -> content::RawJson<String> {
    let current_time = get_current_millisecond();
    let new_menu_resource = MenuResourceAdd{
        name: request.name.to_string(),
        res_type: ResourceType::MENU as i32,
        created_time: current_time,
        updated_time: current_time,
        remark: None,
        path: request.path.to_string(),
        parent_id: request.parentId,
        component: Option::from(request.component.to_string()),
        sort: 0,
        name_zh: request.nameZh.to_string(),
        tree_id_path: "".to_string(),
        code: request.code.to_string()
    };
    // https://stackoverflow.com/questions/72412661/why-the-return-id-was-usize-when-inserting-record
    let menu_id = diesel::insert_into(crate::model::diesel::dolphin::dolphin_schema::menu_resource::table)
        .values(&new_menu_resource)
        .returning(crate::model::diesel::dolphin::dolphin_schema::menu_resource::id)
        .on_conflict_do_nothing()
        .get_results(&mut get_conn())
        .unwrap();
    // update tree id path
    update_tree_id_path(menu_id[0]);
    return box_rest_response("ok");
}

pub fn update_tree_id_path(menu_id: i32){
    use crate::model::diesel::dolphin::dolphin_schema::menu_resource::dsl::*;
    let cte_query_sub_menus = format!(" with recursive sub_menus as
    (
       SELECT
           id,
           parent_id,
           sort,
           tree_id_path
       FROM menu_resource mr
       WHERE id = 5
       UNION ALL
       SELECT
           origin.id,
           origin.parent_id,
           origin.sort,
           (sub_menus.tree_id_path || '-' || origin.id)
       FROM sub_menus
       JOIN menu_resource origin
       ON origin.parent_id = sub_menus.id
    )
    SELECT
        id,
        tree_id_path
    FROM sub_menus
    where id = {}
    ORDER BY sort ASC;
    ",menu_id);
    let cte_menus = sql_query(cte_query_sub_menus)
        .load::<MenuResourcePath>(&mut get_conn())
        .expect("Error find menu resource");
    let predicate = id.eq(menu_id);
    diesel::update(menu_resource.filter(predicate))
        .set(tree_id_path.eq(&cte_menus[0].tree_id_path))
        .get_result::<MenuResource>(&mut get_conn())
        .expect("unable to update new password");
}
