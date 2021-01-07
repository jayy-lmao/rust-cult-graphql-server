use sqlx::postgres::PgPool;
use std::sync::Arc;

use crate::application::services::{CultServices, CultistServices, RitualServices};

use crate::persistance::{CultRepository, CultistRepository, RitualRepository};

pub struct State {
    pub cultist_services: CultistServices,
    pub cult_services: CultServices,
    pub ritual_services: RitualServices,
}

impl State {
    pub fn new(postgres_pool: PgPool) -> Self {
        let cultist_repository = Arc::new(CultistRepository::new(postgres_pool.clone()));
        let cult_repository = Arc::new(CultRepository::new(postgres_pool.clone()));
        let ritual_repository = Arc::new(RitualRepository::new(postgres_pool));

        let cultist_services = CultistServices {
            load_cultists_port: cultist_repository,
        };
        let cult_services = CultServices {
            load_cult_port: cult_repository,
        };
        let ritual_services = RitualServices {
            load_rituals_port: ritual_repository,
        };

        Self {
            cultist_services,
            cult_services,
            ritual_services,
        }
    }
}
