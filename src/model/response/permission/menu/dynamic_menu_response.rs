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
    pub component: Option<String>,
    pub path: String,
    pub parent_id: i32,
    pub routes: Vec<DynamicMenuResponse>
}

impl Default for DynamicMenuResponse {
    fn default() -> Self {
        DynamicMenuResponse {
            id: 0,
            name: "".to_string(),
            res_type: 0,
            created_time: 0,
            updated_time: 0,
            component: None,
            path: "/welcome".to_string(),
            parent_id: 0,
            routes: vec![],

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
            path: p.path.to_string(),
            parent_id: p.parent_id,
            routes: vec![],
            component: p.component.clone()
        }
    }
}
