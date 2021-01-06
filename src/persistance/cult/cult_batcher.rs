use async_trait::async_trait;
use dataloader::BatchFn;
use sqlx::mysql::{MySqlPool, MySqlQueryAs};
use std::{collections::HashMap, result};

use super::cult_model::DBCult;
use crate::persistance::shared::BatchFnLoadError;

pub struct CultBatcher(MySqlPool);
impl CultBatcher {
    pub fn new(mysql_pool: MySqlPool) -> Self {
        Self(mysql_pool)
    }
}
pub type CultBatcherLoadHashMapValue = result::Result<DBCult, BatchFnLoadError>;

#[async_trait]
impl BatchFn<i32, CultBatcherLoadHashMapValue> for CultBatcher {
    async fn load(&self, keys: &[i32]) -> HashMap<i32, CultBatcherLoadHashMapValue> {
        dbg!(format!("load cult by batch {:?}", keys));

        let stmt = format!(
            r#"SELECT Id,
                Address,
                CultBanner,
                CultDescription,
                CultHero,
                CultVideoUrl,
                CultWebsite,
                BrandColour,
                Email,
                Facebook,
                Instagram,
                LinkedIn,
                LogoUrl,
                Mobile,
                Name,
                PostCode,
                ProfileUrl,
                State,
                Suburb,
                Telephone,
                Twitter,
                YoutubeChannel
                FROM Cult WHERE Id in ({})"#,
            keys.iter()
                .map(|i| format!("{}", i))
                .collect::<Vec<String>>()
                .join(",")
        );

        let cults: result::Result<Vec<DBCult>, sqlx::Error> = keys
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
