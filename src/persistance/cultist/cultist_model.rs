use async_graphql::SimpleObject;

#[derive(sqlx::FromRow, Clone, SimpleObject)]
pub struct DBCultist {
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub date_created: chrono::DateTime<chrono::Utc>,
    pub email: Option<String>,
    pub mobile_phone: Option<String>,

}
