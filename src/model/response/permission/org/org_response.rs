use rocket::serde::Deserialize;
use rocket::serde::Serialize;

use crate::model::diesel::dolphin::dolphin_models::Org;

#[derive(Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct OrgResponse {
    pub id: i32,
    pub org_name: String,
    pub parent_id: i32,
    pub disableCheckbox: bool,
    pub tree_id_path: String,
    pub children: Vec<OrgResponse>
}

impl Default for OrgResponse {
    fn default() -> Self {
        OrgResponse {
            id: 0,
            org_name: "".to_string(),
            parent_id: 0,
            disableCheckbox: false,
            tree_id_path: "".to_string(),
            children: vec![]
        }
    }
}

impl From<&Org> for OrgResponse {
    fn from(p: &Org) -> Self {
        Self {
            id: p.id,
            org_name: p.org_name.to_string(),
            parent_id: p.parent_id,
            disableCheckbox: false,
            tree_id_path: p.tree_id_path.to_string(),
            children: vec![]
        }
    }
}
