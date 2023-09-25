use diesel::pg::sql_types::Jsonb;

#[derive(FromSqlRow, AsExpression, serde::Serialize, serde::Deserialize, Debug, Default)]
#[diesel(sql_type = Jsonb)]
pub struct ChannelTag {
    pub id: i32,
    pub name: String,
}

