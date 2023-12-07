use rocket::State;

use crate::{
    domain::LoggedInSession,
    infra::MaudTemplate,
    ui::{profile, FeedProps, ProfileProps},
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

    let props = ProfileProps {
        feed_props,
        me_profile,
    };

    profile::profile_page(props)
}
