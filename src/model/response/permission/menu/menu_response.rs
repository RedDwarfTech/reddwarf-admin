use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rust_wheel::common::util::convert_to_tree::IntoTree;

use crate::model::diesel::dolphin::dolphin_models::MenuResource;

#[derive(Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct MenuResponse {
    pub id: i32,
    pub name: String,
    pub name_zh: String,
    pub parent_id: i32,
    pub sort: i32,
    pub disableCheckbox: bool,
    pub tree_id_path: String,
    pub path: String,
    pub component: Option<String>,
    pub children: Vec<MenuResponse>
}

impl Default for MenuResponse {
    fn default() -> Self {
        MenuResponse {
            id: 0,
            name: "".to_string(),
            name_zh: "".to_string(),
            parent_id: 0,
            sort: 0,
            disableCheckbox: false,
            tree_id_path: "".to_string(),
            path: "".to_string(),
            component: None,
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
            sort: p.sort,
            disableCheckbox: false,
            tree_id_path: p.tree_id_path.to_string(),
            path: p.path.to_string(),
            component: p.component.to_owned(),
            children: vec![]
        }
    }
}

impl IntoTree for &MenuResponse {
    type Output = MenuResponse;

    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_parent_id(&self) -> i32 {
        self.parent_id
    }

    fn convert(&self, children: Vec<Self::Output>) -> Self::Output {
        MenuResponse {
            id: self.id,
            parent_id: self.parent_id,
            sort: self.sort,
            disableCheckbox: false,
            name: self.name.to_string(),
            children,
            name_zh: self.name_zh.to_string(),
            tree_id_path: self.tree_id_path.to_string(),
            path: self.path.to_string(),
            component: self.component.to_owned(),
        }
    }
}