use itertools::Itertools;
use maud::html;
use rocket::State;

use crate::{
    infra::MaudTemplate,
    ui::{feed, nav, page, toki_lili, trending, FeedProps},
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

    page(
        html! {
            div class="main-page" {
                (nav())
                div {
                    (toki_lili())
                    (feed(feed_props))
                }
                (trending())
            }
        },
        "Toki Lili",
    )
}
