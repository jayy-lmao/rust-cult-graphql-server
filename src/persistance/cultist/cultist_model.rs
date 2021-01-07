use async_graphql::SimpleObject;

#[derive(sqlx::FromRow, Clone, SimpleObject)]
pub struct DBCultist {
    pub id: i32,
    #[sqlx(rename = "firstName")]
    pub first_name: Option<String>,

    #[sqlx(rename = "lastName")]
    pub last_name: Option<String>,

    #[sqlx(rename = "dateCreated")]
    pub date_created: chrono::DateTime<chrono::Utc>,

    #[sqlx(rename = "dateUpdated")]
    pub date_updated: chrono::DateTime<chrono::Utc>,

    #[sqlx(rename = "dateDeleted")]
    pub date_deleted: Option<chrono::DateTime<chrono::Utc>>,

    #[sqlx(rename = "email")]
    pub email: Option<String>,

    #[sqlx(rename = "mobilePhone")]
    pub mobile_phone: Option<String>,

}
