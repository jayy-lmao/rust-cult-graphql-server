use async_graphql::{Context, Object};

use crate::domain::Cult;
use crate::domain::Cultist;
use crate::domain::Ritual;

use crate::state::State;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn cultist(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Cultist Id (e.g. A123456)")] cultist_id: String,
    ) -> async_graphql::Result<Option<Cultist>> {
        let cultist_id = cultist_id.replace("A", "").parse::<i32>()?;
        ctx.data_unchecked::<State>()
            .cultist_services
            .get_cultist_by_id(cultist_id)
            .await
    }

    async fn cultists_by_cult_id(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Cult Id (e.g. 123456)")] cult_id: i32,
    ) -> async_graphql::Result<Vec<Cultist>> {
        ctx.data_unchecked::<State>()
            .cultist_services
            .get_cultists_by_cult_id(cult_id)
            .await
    }

    async fn cult(&self, ctx: &Context<'_>, id: i32) -> async_graphql::Result<Option<Cult>> {
        ctx.data_unchecked::<State>()
            .cult_services
            .get_cult_by_id(id)
            .await
    }

    async fn ritual(&self, ctx: &Context<'_>, id: i32) -> async_graphql::Result<Option<Ritual>> {
        ctx.data_unchecked::<State>()
            .ritual_services
            .get_ritual_by_id(id)
            .await
    }
}
