use async_trait::async_trait;
use dataloader::BatchFn;
use sqlx::postgres::{PgPool, PgQueryAs};
use std::{collections::HashMap, result};

use super::cultist_model::DBCultist;
use crate::persistance::shared::BatchFnLoadError;

pub struct CultistByIdBatcher(PgPool);
impl CultistByIdBatcher {
    pub fn new(postgres_pool: PgPool) -> Self {
        Self(postgres_pool)
    }
}

pub type CultistByIdBatcherLoadHashMapValue = result::Result<DBCultist, BatchFnLoadError>;

#[async_trait]
impl BatchFn<i32, CultistByIdBatcherLoadHashMapValue> for CultistByIdBatcher {
    async fn load(&self, keys: &[i32]) -> HashMap<i32, CultistByIdBatcherLoadHashMapValue> {
        dbg!(format!("load cultist by batch {:?}", keys));

        let stmt = format!(
            r#"SELECT id,
                date_created,
                date_deleted,
                date_updated,
                email,
                first_name, 
                last_name,
                mobile_phone 
                FROM cultists WHERE id in ({})"#,
            keys.iter()
                .map(|i| format!("{}", i))
                .collect::<Vec<String>>()
                .join(",")
        );

        let cultists: result::Result<Vec<DBCultist>, sqlx::Error> = keys
            .iter()
            .fold(sqlx::query_as(&stmt), |q, key| q.bind(key))
            .fetch_all(&self.0)
            .await;

        match cultists {
            Ok(cultists) => {
                let cultists_map = cultists
                    .into_iter()
                    .map(|cultist| (cultist.id, Ok(cultist)))
                    .collect();

                keys.iter().fold(
                    cultists_map,
                    |mut map: HashMap<i32, CultistByIdBatcherLoadHashMapValue>, key| {
                        map.entry(*key).or_insert(Err(BatchFnLoadError::NotFound));
                        map
                    },
                )
            }
            Err(e) => keys
                .iter()
                .map(|k| (*k, Err(BatchFnLoadError::DBError(e.to_string()))))
                .collect(),
        }
    }
}
