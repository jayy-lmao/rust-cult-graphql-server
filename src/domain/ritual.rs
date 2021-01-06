use async_graphql::SimpleObject;

#[derive(Clone, SimpleObject)]
pub struct Ritual {
    pub id: i32,
    pub ritual_type: Option<String>,
}
