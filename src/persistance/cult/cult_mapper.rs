use super::cult_model::DBCult;
use crate::domain::{Cult, CultAddress, CultContactDetails, Profile};

pub fn map_db_cultist_to_cult(db_cult: DBCult) -> Cult {
    Cult {
        id: db_cult.id,
        name: db_cult.name,
        cult_address: CultAddress {
            address: db_cult.address,
            postcode: db_cult.postcode,
            state: db_cult.state,
        },
        profile: Profile {
            cult_description: db_cult.cult_description,
            brand_colour: db_cult.brand_colour,
            logo_url: db_cult.logo_url,
            cult_website: db_cult.cult_website,
        },
        cult_contact_details: CultContactDetails {
            email: db_cult.email,
        },
    }
}
