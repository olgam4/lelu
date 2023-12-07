use itertools::Itertools;
use maud::html;
use rocket::State;

use crate::{
    domain::{Lili, Profile},
    infra::MaudTemplate,
    ui::{feed, nav, page, profile, FeedProps},
    AppState,
};

#[get("/profile")]
pub fn profile_page(state: &State<AppState>) -> MaudTemplate {
    let some_profile = state
        .persist
        .load::<Profile>(format!("profile_gamachexx").as_str())
        .unwrap();

    let lilis = state
        .persist
        .list()
        .unwrap()
        .iter()
        .filter(|key| key.starts_with("lili_"))
        .map(|key| state.persist.load::<Lili>(key).unwrap())
        .sorted_by(|a, b| b.timestamp.cmp(&a.timestamp))
        .collect::<Vec<Lili>>();

    let feed_props = FeedProps {
        lilis: lilis
            .iter()
            .map(|c| (c.to_owned(), some_profile.clone()))
            .collect(),
    };

    page(
        html! {
            div class="main-page" {
                (nav())
                div {
                    (profile(some_profile.clone()))
                    (feed(feed_props))
                }
                div {}
            }
        },
        "Profile",
    )
}
