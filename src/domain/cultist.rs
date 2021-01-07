use super::ContactInformation;

use async_graphql::*;

#[derive(Clone, SimpleObject)]
pub struct Cultist {
    pub id: String,
    pub contact_information: ContactInformation,
    pub date_created: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}
