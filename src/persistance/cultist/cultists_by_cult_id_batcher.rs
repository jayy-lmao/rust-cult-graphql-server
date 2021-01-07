use async_trait::async_trait;
use dataloader::BatchFn;
use sqlx::mysql::{MySqlPool, MySqlQueryAs};
use std::{collections::HashMap, result};

use super::cultist_model::DBCultist;
use crate::persistance::shared::BatchFnLoadError;

pub struct CultistsByCultIdBatcher(MySqlPool);

impl CultistsByCultIdBatcher {
    pub fn new(mysql_pool: MySqlPool) -> Self {
        Self(mysql_pool)
    }
}
pub type CultistsVecByCultIdBatcherLoadHashMapValue =
    result::Result<Vec<DBCultist>, BatchFnLoadError>;

#[async_trait]
impl BatchFn<i32, CultistsVecByCultIdBatcherLoadHashMapValue> for CultistsByCultIdBatcher {
    async fn load(&self, keys: &[i32]) -> HashMap<i32, CultistsVecByCultIdBatcherLoadHashMapValue> {
        dbg!(format!("load cultist by batch {:?}", keys));

        // Because a cultist can be a member of multiple cults
        let stmt = format!(
            r#"SELECT 
                firstName,
                lastName,
                dateCreated,
                dateUpdated,
                dateDeleted,
                email,
                mobilePhone,
                cultistId as Id,
                cultId,
                role
              FROM cultists
              INNER JOIN cultistCults
              ON cultistCult.cultistId = cultist.Id 
              WHERE cultistCult.cultId in ({})"#,
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
