use async_graphql::Result;
use async_trait::async_trait;
use dataloader::cached::Loader;
use sqlx::postgres::PgPool;

use crate::{
    application::ports::outgoing::LoadCultistsPort, domain::Cultist,
    persistance::shared::BatchFnLoadError,
};

use super::{
    cultist_by_id_batcher::{CultistByIdBatcher, CultistByIdBatcherLoadHashMapValue},
    cultist_mapper::map_db_cultist_to_cultist,
    cultists_by_cult_id_batcher::{
        CultistsByCultIdBatcher, CultistsVecByCultIdBatcherLoadHashMapValue,
    },
};

pub struct CultistRepository {
    cultist_by_id_loader: Loader<i32, CultistByIdBatcherLoadHashMapValue, CultistByIdBatcher>,
    cultists_by_cult_id_loader:
        Loader<i32, CultistsVecByCultIdBatcherLoadHashMapValue, CultistsByCultIdBatcher>,
}
impl CultistRepository {
    pub fn new(postgres_pool: PgPool) -> Self {
        Self {
            cultist_by_id_loader: Loader::new(CultistByIdBatcher::new(postgres_pool.clone())),
            cultists_by_cult_id_loader: Loader::new(CultistsByCultIdBatcher::new(postgres_pool)),
        }
    }
}

#[async_trait]
impl LoadCultistsPort for CultistRepository {
    async fn load_cultist(&self, id: i32) -> Result<Option<Cultist>> {
        match self.cultist_by_id_loader.load(id).await {
            Ok(db_cultist) => Ok(Some(map_db_cultist_to_cultist(db_cultist))),
            Err(err) => match err {
                BatchFnLoadError::NotFound => Ok(None),
                BatchFnLoadError::DBError(db_err) => Err(db_err.into()),
            },
        }
    }
    async fn load_cultists_by_cult_id(&self, cult_id: i32) -> Result<Vec<Cultist>> {
        match self.cultists_by_cult_id_loader.load(cult_id).await {
            Ok(cultists) => Ok(cultists
                .into_iter()
                .map(map_db_cultist_to_cultist)
                .collect()),
            Err(err) => match err {
                BatchFnLoadError::NotFound => Ok(Vec::new()),
                BatchFnLoadError::DBError(db_err) => Err(db_err.into()),
            },
        }
    }
}
