use async_graphql::Result;
use std::sync::Arc;

use crate::{application::ports::outgoing::LoadCultsPort, domain::Cult};

pub struct CultServices {
    pub load_cult_port: Arc<dyn LoadCultsPort + Send + Sync>,
}
impl CultServices {
    pub async fn get_cult_by_id(&self, id: i32) -> Result<Option<Cult>> {
        self.load_cult_port.load_cult(id).await
    }
}
