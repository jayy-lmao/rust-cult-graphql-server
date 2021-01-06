use async_graphql::Result;
use async_trait::async_trait;
use dataloader::cached::Loader;
use sqlx::mysql::MySqlPool;

use super::{
    cult_batcher::{CultBatcher, CultBatcherLoadHashMapValue},
    cult_mapper::map_db_cultist_to_cult,
};
use crate::{
    application::ports::outgoing::LoadCultsPort, domain::Cult,
    persistance::shared::BatchFnLoadError,
};

pub struct CultRepository {
    cult_loader: Loader<i32, CultBatcherLoadHashMapValue, CultBatcher>,
}
impl CultRepository {
    pub fn new(mysql_pool: MySqlPool) -> Self {
        Self {
            cult_loader: Loader::new(CultBatcher::new(mysql_pool)),
        }
    }
}

#[async_trait]
impl LoadCultsPort for CultRepository {
    async fn load_cult(&self, id: i32) -> Result<Option<Cult>> {
        match self.cult_loader.load(id).await {
            Ok(db_cult) => Ok(Some(map_db_cultist_to_cult(db_cult))),
            Err(err) => match err {
                BatchFnLoadError::NotFound => Ok(None),
                BatchFnLoadError::DBError(db_err) => Err(db_err.into()),
            },
        }
    }
}
