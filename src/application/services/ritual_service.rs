use async_graphql::Result;
use std::sync::Arc;

use crate::{application::ports::outgoing::LoadRitualsPort, domain::Ritual};

pub struct RitualServices {
    pub load_rituals_port: Arc<dyn LoadRitualsPort + Send + Sync>,
}
impl RitualServices {
    pub async fn get_ritual_by_id(&self, id: i32) -> Result<Option<Ritual>> {
        self.load_rituals_port.load_ritual(id).await
    }
}
