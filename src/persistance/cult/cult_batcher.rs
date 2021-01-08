use async_trait::async_trait;
use dataloader::BatchFn;
use sqlx::postgres::{PgPool, PgQueryAs};
use std::{collections::HashMap, result};

use super::cult_entity::CultEntity;
use crate::persistance::shared::BatchFnLoadError;

pub struct CultBatcher(PgPool);
impl CultBatcher {
    pub fn new(postgres_pool: PgPool) -> Self {
        Self(postgres_pool)
    }
}
pub type CultBatcherLoadHashMapValue = result::Result<CultEntity, BatchFnLoadError>;

#[async_trait]
impl BatchFn<i32, CultBatcherLoadHashMapValue> for CultBatcher {
    async fn load(&self, keys: &[i32]) -> HashMap<i32, CultBatcherLoadHashMapValue> {
        dbg!(format!("load cult by batch {:?}", keys));

        let stmt = format!(
            r#"SELECT id,
                address,
                cult_description,
                cult_website,
                brand_colour,
                logo_url,
                email,
                name,
                postcode,
                state
                FROM cults WHERE id in ({})"#,
            keys.iter()
                .map(|i| format!("{}", i))
                .collect::<Vec<String>>()
                .join(",")
        );

        let cults: result::Result<Vec<CultEntity>, sqlx::Error> = keys
            .iter()
            .fold(sqlx::query_as(&stmt), |q, key| q.bind(key))
            .fetch_all(&self.0)
            .await;

        match cults {
            Ok(cults) => {
                let cults_map = cults
                    .into_iter()
                    .map(|cults| (cults.id, Ok(cults)))
                    .collect();

                keys.iter().fold(
                    cults_map,
                    |mut map: HashMap<i32, CultBatcherLoadHashMapValue>, key| {
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
