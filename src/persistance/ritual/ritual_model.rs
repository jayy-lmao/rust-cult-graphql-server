use async_graphql::SimpleObject;

#[derive(sqlx::FromRow, Clone, SimpleObject)]
pub struct DBRitual {
    pub id: i32,

    #[sqlx(rename = "ritualType")]
    pub ritual_type: Option<String>,
}
