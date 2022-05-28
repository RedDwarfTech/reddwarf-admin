use rust_wheel::config::db::config;

use crate::service::permission::menu::menu_service::update_tree_id_path;

#[test]
fn test_add() {
    let connection = config::establish_connection();
    update_tree_id_path(24,connection);
    assert_eq!(add(1, 2), 3);
}


pub fn add(a: i32, b: i32) -> i32 {
    a + b
}