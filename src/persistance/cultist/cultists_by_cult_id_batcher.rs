use async_trait::async_trait;
use dataloader::BatchFn;
use sqlx::postgres::{PgPool, PgQueryAs};
use std::{collections::HashMap, result};

use super::cultist_entity::CultistEntity;
use crate::persistance::shared::BatchFnLoadError;

pub struct CultistsByCultIdBatcher(PgPool);

impl CultistsByCultIdBatcher {
    pub fn new(postgres_pool: PgPool) -> Self {
        Self(postgres_pool)
    }
}
pub type CultistsVecByCultIdBatcherLoadHashMapValue =
    result::Result<Vec<CultistEntity>, BatchFnLoadError>;

#[async_trait]
impl BatchFn<i32, CultistsVecByCultIdBatcherLoadHashMapValue> for CultistsByCultIdBatcher {
    async fn load(&self, keys: &[i32]) -> HashMap<i32, CultistsVecByCultIdBatcherLoadHashMapValue> {
        dbg!(format!("load cultist by batch {:?}", keys));

        // Because a cultist can be a member of multiple cults
        let stmt = format!(
            r#"SELECT 
                first_name,
                last_name,
                date_created,
                email,
                mobile_phone,
                cultist_id as id,
                cult_id
              FROM cultists
              INNER JOIN cultist_cults
              ON cultist_cults.cultist_id = cultists.id 
              WHERE cultist_cults.cult_id in ({})"#,
            keys.iter()
                .map(|i| format!("{}", i))
                .collect::<Vec<String>>()
                .join(",")
        );

        let cultists: result::Result<Vec<CultistEntity>, sqlx::Error> = keys
            .iter()
            .fold(sqlx::query_as(&stmt), |q, key| q.bind(key))
            .fetch_all(&self.0)
            .await;

        match cultists {
            Ok(cultists) => {
                let mut map = HashMap::new();
                keys.iter().for_each(|&key| {
                    let mut cons = Vec::new();
                    cultists.iter().for_each(|c| cons.push(c.clone()));
                    map.insert(key, Ok(cons));
                });
                map
            }
            Err(e) => keys
                .iter()
                .map(|k| (*k, Err(BatchFnLoadError::DBError(e.to_string()))))
                .collect(),
        }
    }
}
