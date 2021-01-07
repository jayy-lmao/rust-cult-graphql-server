use async_graphql::{Context, Object, SimpleObject};

use crate::{domain::Cultist, State};

#[derive(Clone, SimpleObject)]
pub struct Profile {
    pub brand_colour: Option<String>,
    pub logo_url: Option<String>,
    pub cult_description: Option<String>,
    pub cult_website: Option<String>,
}
#[derive(Clone, SimpleObject)]
pub struct CultContactDetails {
    pub email: Option<String>,
}

#[derive(Clone, SimpleObject)]
pub struct CultAddress {
    pub address: Option<String>,
    pub postcode: Option<String>,
    pub state: Option<String>,
}

pub struct Cult {
    pub id: i32,
    pub name: String,

    pub profile: Profile,
    pub cult_address: CultAddress,
    pub cult_contact_details: CultContactDetails,
}

#[Object]
impl Cult {
    /// Id of the Cult (e.g. 54321)
    async fn id(&self) -> i32 {
        self.id
    }
    async fn name(&self) -> &String {
        &self.name
    }
    async fn profile(&self) -> &Profile {
        &self.profile
    }
    async fn cult_address(&self) -> &CultAddress {
        &self.cult_address
    }
    async fn cult_contact_details(&self) -> &CultContactDetails {
        &self.cult_contact_details
    }
    async fn cultists(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Cultist>> {
        ctx.data_unchecked::<State>()
            .cultist_services
            .get_cultists_by_cult_id(self.id)
            .await
    }
}
