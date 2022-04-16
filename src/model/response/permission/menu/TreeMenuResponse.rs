#[derive(Insertable,Queryable,Debug,Serialize,Deserialize,Default)]
pub struct TreeMenuResource {
    pub id: i32,
    pub name: String,
    pub res_type: i32,
    pub created_time: i64,
    pub updated_time: i64,
    pub remark: Option<String>,
    pub path: Option<String>,
    pub parent_id: i32,
    pub children: Option<List<TreeMenuResource>>
}