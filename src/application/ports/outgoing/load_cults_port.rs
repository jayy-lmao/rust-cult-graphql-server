use async_graphql::Result;
use async_trait::async_trait;

use crate::domain::Cult;

#[async_trait]
pub trait LoadCultsPort {
    async fn load_cult(&self, id: i32) -> Result<Option<Cult>>;
}
