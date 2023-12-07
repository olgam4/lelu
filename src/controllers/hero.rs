use itertools::Itertools;
use rocket::State;

use crate::{
    infra::MaudTemplate,
    ui::{FeedProps, HeroProps},
    AppServices,
};

#[get("/")]
pub fn hero(services: &State<AppServices>) -> MaudTemplate {
    let lilis = services.lili_service.get_all_lilis();

    let lilis_with_their_profiles = lilis
        .into_iter()
        .map(|lili| {
            let profile = services
                .profile_service
                .get_profile(&lili.username)
                .unwrap();
            (lili, profile)
        })
        .collect_vec();

    let feed_props = FeedProps {
        lilis: lilis_with_their_profiles,
    };

    let props = HeroProps {
        feed_props,
    };

    crate::ui::hero::hero(props)
}
