use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use crate::model::diesel::dolphin::dolphin_models::MenuResource;

#[derive(Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct MenuResponse {
    pub id: i32,
    pub name: String,
    pub name_zh: String,
    pub parent_id: i32,
    pub disableCheckbox: bool,
    pub tree_id_path: String,
    pub children: Vec<MenuResponse>
}

impl Default for MenuResponse {
    fn default() -> Self {
        MenuResponse {
            id: 0,
            name: "".to_string(),
            name_zh: "".to_string(),
            parent_id: 0,
            disableCheckbox: false,
            tree_id_path: "".to_string(),
            children: vec![]
        }
    }
}

impl From<&MenuResource> for MenuResponse {
    fn from(p: &MenuResource) -> Self {
        Self {
            id: p.id,
            name: p.name.to_string(),
            name_zh: p.name_zh.to_string(),
            parent_id: p.parent_id,
            disableCheckbox: false,
            tree_id_path: p.tree_id_path.to_string(),
            children: vec![]
        }
    }
}
