use async_graphql::Result;
use async_trait::async_trait;

use crate::domain::Cultist;

#[async_trait]
pub trait LoadCultistsPort {
    async fn load_cultist(&self, id: i32) -> Result<Option<Cultist>>;
    async fn load_cultists_by_cult_id(&self, cult_id: i32) -> Result<Vec<Cultist>>;
}
