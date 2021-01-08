use async_graphql::SimpleObject;

#[derive(sqlx::FromRow, Clone, SimpleObject)]
pub struct RitualEntity {
    pub id: i32,
    pub ritual_type: Option<String>,
}
