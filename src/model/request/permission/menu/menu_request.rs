use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use crate::model::request::permission::menu::role_menu_request::RoleMenuRequest;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct MenuRequest {
    pub pageNum: i64,
    pub pageSize: i64,
    pub parentId: i32,
}

impl From<&RoleMenuRequest> for MenuRequest {
    fn from(p: &RoleMenuRequest) -> Self {
        Self {
            pageNum: p.pageNum,
            pageSize: p.pageSize,
            parentId: p.parentId
        }
    }
}
