use async_graphql::SimpleObject;

#[derive(sqlx::FromRow, Clone, SimpleObject)]
pub struct DBCultist {
    #[sqlx(rename = "Id")]
    pub id: i32,

    #[sqlx(rename = "FirstName")]
    pub first_name: Option<String>,

    #[sqlx(rename = "LastName")]
    pub last_name: Option<String>,

    #[sqlx(rename = "DateCreated")]
    pub date_created: chrono::DateTime<chrono::Utc>,

    #[sqlx(rename = "DateUpdated")]
    pub date_updated: chrono::DateTime<chrono::Utc>,

    #[sqlx(rename = "DateDeleted")]
    pub date_deleted: Option<chrono::DateTime<chrono::Utc>>,

    #[sqlx(rename = "Email")]
    pub email: Option<String>,

    #[sqlx(rename = "MobilePhone")]
    pub mobile_phone: Option<String>,

    #[sqlx(rename = "MergedTo")]
    pub merged_to: Option<String>,

    #[sqlx(rename = "MergedToDate")]
    pub merged_to_date: Option<chrono::DateTime<chrono::Utc>>,
}
