use async_graphql::SimpleObject;

#[derive(sqlx::FromRow, Clone, SimpleObject)]
pub struct DBCult {
    pub id: i32,
    pub name: String,
    pub state: Option<String>,
    pub postcode: Option<String>,
    pub brand_colour: Option<String>,
    pub logo_url: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub cult_website: Option<String>,
    pub cult_description: Option<String>,
}
