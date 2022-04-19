use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use crate::model::diesel::dolphin::dolphin_models::MenuResource;

#[derive(Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct DynamicMenuResponse {
    pub id: i32,
    pub name: String,
    pub res_type: i32,
    pub created_time: i64,
    pub updated_time: i64,
    pub icon: String,
    pub component: String,
    pub path: String,
    pub parent_id: i32,
    pub children: Vec<DynamicMenuResponse>
}

impl Default for DynamicMenuResponse {
    fn default() -> Self {
        DynamicMenuResponse {
            id: 0,
            name: "".to_string(),
            res_type: 0,
            created_time: 0,
            updated_time: 0,
            icon: "smile".to_string(),
            path: "/welcome".to_string(),
            parent_id: 0,
            children: vec![],
            component: "./Welcome".to_string()
        }
    }
}

impl From<&MenuResource> for DynamicMenuResponse {
    fn from(p: &MenuResource) -> Self {
        Self {
            id: p.id,
            name: p.name.to_string(),
            res_type: 0,
            created_time: 0,
            updated_time: 0,
            icon: "smile".to_string(),
            path: "/welcome".to_string(),
            parent_id: p.parent_id,
            children: vec![],
            component: "./Welcome".to_string()
        }
    }
}
