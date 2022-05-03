use rocket::serde::Deserialize;
use rocket::serde::Serialize;

use crate::model::response::permission::menu::menu_response::MenuResponse;

#[derive(Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct OrgResponseWrapper {
    pub orgs: Vec<OrgResponse>,
    pub checked_keys: Vec<String>
}

impl Default for OrgResponseWrapper {
    fn default() -> Self {
        OrgResponseWrapper {
            orgs: vec![],
            checked_keys: vec![]
        }
    }
}
