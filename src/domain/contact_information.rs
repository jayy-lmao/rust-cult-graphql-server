use async_graphql::*;

#[derive(Clone, SimpleObject)]
pub struct ContactInformation {
    pub email: Option<String>,
    pub mobile_phone: Option<String>,
}
