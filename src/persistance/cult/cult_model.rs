use async_graphql::SimpleObject;

#[derive(sqlx::FromRow, Clone, SimpleObject)]
pub struct DBCult {
    #[sqlx(rename = "Id")]
    pub id: i32,

    #[sqlx(rename = "Name")]
    pub name: String,

    #[sqlx(rename = "Suburb")]
    pub suburb: Option<String>,

    #[sqlx(rename = "State")]
    pub state: Option<String>,

    #[sqlx(rename = "PostCode")]
    pub post_code: Option<String>,

    #[sqlx(rename = "ProfileUrl")]
    pub profile_url: Option<String>,

    #[sqlx(rename = "BrandColour")]
    pub brand_colour: Option<String>,

    #[sqlx(rename = "LogoUrl")]
    pub logo_url: Option<String>,

    #[sqlx(rename = "Email")]
    pub email: Option<String>,

    #[sqlx(rename = "Telephone")]
    pub telephone: Option<String>,

    #[sqlx(rename = "Mobile")]
    pub mobile: Option<String>,

    #[sqlx(rename = "Address")]
    pub address: Option<String>,

    #[sqlx(rename = "CultBanner")]
    pub cult_banner: Option<String>,

    #[sqlx(rename = "CultWebsite")]
    pub cult_website: Option<String>,

    #[sqlx(rename = "Facebook")]
    pub facebook: Option<String>,

    #[sqlx(rename = "LinkedIn")]
    pub linkedin: Option<String>,

    #[sqlx(rename = "Instagram")]
    pub instagram: Option<String>,

    #[sqlx(rename = "Twitter")]
    pub twitter: Option<String>,

    #[sqlx(rename = "YoutubeChannel")]
    pub youtube_channel: Option<String>,

    #[sqlx(rename = "CultDescription")]
    pub cult_description: Option<String>,

    #[sqlx(rename = "CultHero")]
    pub cult_hero: Option<String>,

    #[sqlx(rename = "CultVideoUrl")]
    pub cult_video_url: Option<String>,
}
