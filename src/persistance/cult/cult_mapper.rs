use super::cult_model::DBCult;
use crate::domain::{Cult, CultAddress, CultContactDetails, Profile, SocialMedia};

pub fn map_db_cultist_to_cult(db_cult: DBCult) -> Cult {
    Cult {
        id: db_cult.id,
        name: db_cult.name,
        cult_address: CultAddress {
            address: db_cult.address,
            postcode: db_cult.post_code,
            state: db_cult.state,
            suburb: db_cult.suburb,
        },
        profile: Profile {
            cult_description: db_cult.cult_description,
            brand_colour: db_cult.brand_colour,
            logo_url: db_cult.logo_url,
            profile_url: db_cult.profile_url,
            cult_video_url: db_cult.cult_video_url,
            cult_website: db_cult.cult_website,
            cult_hero: db_cult.cult_hero,
        },
        cult_contact_details: CultContactDetails {
            email: db_cult.email,
            mobile: db_cult.mobile,
            telephone: db_cult.telephone,
        },
        social_media: SocialMedia {
            facebook: db_cult.facebook,
            linkedin: db_cult.linkedin,
            instagram: db_cult.instagram,
            twitter: db_cult.twitter,
            youtube_channel: db_cult.youtube_channel,
        },
    }
}
