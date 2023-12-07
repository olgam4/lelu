use maud::html;
use rocket::State;

use crate::{
    domain::LoggedInSession,
    infra::MaudTemplate,
    ui::{feed, nav, page, profile, FeedProps},
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

    page(
        html! {
            div class="main-page" {
                (nav())
                div {
                    (profile(me_profile.clone()))
                    (feed(feed_props))
                }
                div {}
            }
        },
        "Profile",
    )
}
