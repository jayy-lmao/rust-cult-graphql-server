use super::cultist_entity::CultistEntity;
use crate::domain::{ContactInformation, Cultist};

pub fn map_db_cultist_to_cultist(db_cultist: CultistEntity) -> Cultist {
    Cultist {
        id: format!("C{}", db_cultist.id),
        date_created: db_cultist.date_created.to_string(),
        first_name: db_cultist.first_name,
        last_name: db_cultist.last_name,
        contact_information: ContactInformation {
            email: db_cultist.email,
            mobile_phone: db_cultist.mobile_phone,
        },
    }
}
