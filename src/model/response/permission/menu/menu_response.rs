use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use crate::model::diesel::dolphin::dolphin_models::MenuResource;

#[derive(Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct MenuResponse {
    pub id: i32,
    pub name: String,
    pub res_type: i32,
    pub created_time: i64,
    pub updated_time: i64,
    pub remark: Option<String>,
    pub path: Option<String>,
    pub parent_id: i32,
    pub children: Vec<MenuResponse>
}

impl Default for MenuResponse {
    fn default() -> Self {
        MenuResponse {
            id: 0,
            name: "".to_string(),
            res_type: 0,
            created_time: 0,
            updated_time: 0,
            remark: None,
            path: None,
            parent_id: 0,
            children: vec![]
        }
    }
}

impl From<&MenuResource> for MenuResponse {
    fn from(p: &MenuResource) -> Self {
        Self {
            id: p.id,
            name: p.name.to_string(),
            res_type: 0,
            created_time: 0,
            updated_time: 0,
            remark: None,
            path: None,
            parent_id: p.parent_id,
            children: vec![]
        }
    }
}
