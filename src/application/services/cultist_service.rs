use async_graphql::Result;
use std::sync::Arc;

use crate::{application::ports::outgoing::LoadCultistsPort, domain::Cultist};

pub struct CultistServices {
    pub load_cultists_port: Arc<dyn LoadCultistsPort + Send + Sync>,
}
impl CultistServices {
    pub async fn get_cultist_by_id(&self, id: i32) -> Result<Option<Cultist>> {
        self.load_cultists_port.load_cultist(id).await
    }
    pub async fn get_cultists_by_cult_id(&self, id: i32) -> Result<Vec<Cultist>> {
        self.load_cultists_port.load_cultists_by_cult_id(id).await
    }
}
