use async_trait::async_trait;
use dataloader::BatchFn;
use sqlx::mysql::{MySqlPool,MySqlQueryAs};
use std::{collections::HashMap, result};

use super::ritual_model::DBRitual;
use crate::persistance::shared::BatchFnLoadError;

pub struct RitualBatcher(MySqlPool);

impl RitualBatcher {
    pub fn new(mysql_pool: MySqlPool) -> Self {
        Self(mysql_pool)
    }
}
pub type RitualBatcherLoadHashMapValue = result::Result<DBRitual, BatchFnLoadError>;
#[async_trait]
impl BatchFn<i32, RitualBatcherLoadHashMapValue> for RitualBatcher {
    async fn load(&self, keys: &[i32]) -> HashMap<i32, RitualBatcherLoadHashMapValue> {
        dbg!(format!("load cultist by batch {:?}", keys));

        let stmt = format!(
            r#"SELECT id,
                ritualType 
                FROM rituals WHERE id in ({})"#,
            keys.iter()
                .map(|i| format!("{}", i))
                .collect::<Vec<String>>()
                .join(",")
        );

        let rituals: result::Result<Vec<DBRitual>, sqlx::Error> = keys
            .iter()
            .fold(sqlx::query_as(&stmt), |q, key| q.bind(key))
            .fetch_all(&self.0)
            .await;

        match rituals {
            Ok(rituals) => {
                let rituals_map = rituals
                    .into_iter()
                    .map(|ritual| (ritual.id, Ok(ritual)))
                    .collect();

                keys.iter().fold(
                    rituals_map,
                    |mut map: HashMap<i32, RitualBatcherLoadHashMapValue>, key| {
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
