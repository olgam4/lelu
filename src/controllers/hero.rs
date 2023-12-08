use itertools::Itertools;
use rocket::State;

use crate::{
    domain::CurrentSession,
    infra::MaudTemplate,
    ui::{FeedProps, HeroProps, NavProps},
    AppServices,
};

#[get("/")]
pub fn hero(services: &State<AppServices>, current_session: CurrentSession) -> MaudTemplate {
    let username = match current_session.username {
        Some(ref username) => username.clone(),
        None => "".to_string(),
    };
    let lilis = services.lili_service.get_all_lilis(&username);

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

    let nav_props = NavProps {
        is_logged_in: current_session.username.is_some(),
    };

    let props = HeroProps {
        feed_props,
        nav_props,
    };

    crate::ui::hero::hero(props)
}
