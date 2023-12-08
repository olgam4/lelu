use rocket::State;

use crate::{
    domain::{CurrentSession, LoggedInSession},
    infra::MaudTemplate,
    ui::{profile, FeedProps, NavProps, ProfileProps},
    AppServices,
};

#[get("/me")]
pub fn profile_page(
    services: &State<AppServices>,
    current_session: LoggedInSession,
) -> MaudTemplate {
    let me_profile = services
        .profile_service
        .get_profile(&current_session.username)
        .unwrap();

    let lilis = services
        .lili_service
        .get_all_lilis_from_user(&me_profile.username)
        .unwrap();

    let feed_props = FeedProps {
        lilis: lilis
            .into_iter()
            .map(|lili| (lili.lili, lili.profile))
            .collect(),
    };

    let nav_props = NavProps { is_logged_in: true };

    let props = ProfileProps {
        feed_props,
        me_profile,
        nav_props,
    };

    profile::profile_page(props)
}

#[get("/profile/<username>")]
pub fn specific_profile_page(
    services: &State<AppServices>,
    username: String,
    current_session: CurrentSession,
) -> MaudTemplate {
    let the_profile = services.profile_service.get_profile(&username).unwrap();

    let lilis = services
        .lili_service
        .get_all_lilis_from_user(&the_profile.username)
        .unwrap();

    let feed_props = FeedProps {
        lilis: lilis
            .into_iter()
            .map(|lili| (lili.lili, lili.profile))
            .collect(),
    };

    let nav_props = NavProps {
        is_logged_in: current_session.username.is_some(),
    };

    let props = ProfileProps {
        feed_props,
        me_profile: the_profile,
        nav_props,
    };

    profile::profile_page(props)
}
