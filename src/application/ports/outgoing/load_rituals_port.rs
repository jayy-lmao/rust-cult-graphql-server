use async_graphql::Result;
use async_trait::async_trait;

use crate::domain::Ritual;

#[async_trait]
pub trait LoadRitualsPort {
    async fn load_ritual(&self, id: i32) -> Result<Option<Ritual>>;
}
