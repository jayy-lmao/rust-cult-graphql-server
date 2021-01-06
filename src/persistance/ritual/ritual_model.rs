use async_graphql::SimpleObject;

#[derive(sqlx::FromRow, Clone, SimpleObject)]
pub struct DBRitual {
    #[sqlx(rename = "Id")]
    pub id: i32,

    #[sqlx(rename = "RitualType")]
    pub ritual_type: Option<String>,
}
