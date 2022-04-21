use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use crate::model::diesel::dolphin::dolphin_models::MenuResource;
use crate::model::response::permission::menu::menu_response::MenuResponse;

#[derive(Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct MenuResponseWrapper {
    pub menus: Vec<MenuResponse>,
    pub checked_keys: Vec<i32>
}

impl Default for MenuResponseWrapper {
    fn default() -> Self {
        MenuResponseWrapper {
            menus: vec![],
            checked_keys: vec![]
        }
    }
}
