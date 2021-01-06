use super::ritual_model::DBRitual;
use crate::domain::Ritual;

pub fn map_db_ritual_to_ritual(db_ritual: DBRitual) -> Ritual {
    Ritual {
        id: db_ritual.id,
        ritual_type: db_ritual.ritual_type,
    }
}
