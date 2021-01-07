use super::ritual_batcher::{RitualBatcher, RitualBatcherLoadHashMapValue};
use crate::application::ports::outgoing::LoadRitualsPort;
use crate::domain::Ritual;
use async_graphql::Result;
use async_trait::async_trait;
use dataloader::cached::Loader;
use sqlx::postgres::PgPool;

use super::ritual_mapper::map_db_ritual_to_ritual;
use crate::persistance::shared::BatchFnLoadError;

pub struct RitualRepository {
    ritual_loader: Loader<i32, RitualBatcherLoadHashMapValue, RitualBatcher>,
}
impl RitualRepository {
    pub fn new(postgres_pool: PgPool) -> Self {
        Self {
            ritual_loader: Loader::new(RitualBatcher::new(postgres_pool)),
        }
    }
}

#[async_trait]
impl LoadRitualsPort for RitualRepository {
    async fn load_ritual(&self, id: i32) -> Result<Option<Ritual>> {
        match self.ritual_loader.load(id).await {
            Ok(db_ritual) => Ok(Some(map_db_ritual_to_ritual(db_ritual))),
            Err(err) => match err {
                BatchFnLoadError::NotFound => Ok(None),
                BatchFnLoadError::DBError(db_err) => Err(db_err.into()),
            },
        }
    }
}
