use async_graphql::SimpleObject;

#[derive(sqlx::FromRow, Clone, SimpleObject)]
pub struct DBCult {
    pub id: i32,
    pub name: String,
    pub state: Option<String>,

    #[sqlx(rename = "postCode")]
    pub post_code: Option<String>,

    #[sqlx(rename = "brandColour")]
    pub brand_colour: Option<String>,

    #[sqlx(rename = "logoUrl")]
    pub logo_url: Option<String>,

    pub email: Option<String>,

    pub address: Option<String>,

    #[sqlx(rename = "cultWebsite")]
    pub cult_website: Option<String>,

    #[sqlx(rename = "cultDescription")]
    pub cult_description: Option<String>,
}
