use itertools::Itertools;
use maud::html;
use rocket::State;

use crate::{
    domain::{Lili, Profile},
    infra::MaudTemplate,
    ui::{feed, nav, page, toki_lili, trending, FeedProps},
    AppState,
};

#[get("/")]
pub fn hello(state: &State<AppState>) -> MaudTemplate {
    let some_profile = state.persist.load::<Profile>("profile_gamachexx").unwrap();

    let lilis = state
        .persist
        .list()
        .unwrap()
        .iter()
        .filter(|key| key.starts_with("lili_"))
        .map(|key| state.persist.load::<Lili>(key).unwrap())
        .collect::<Vec<Lili>>();

    let lilis: Vec<Lili> = lilis
        .into_iter()
        .sorted_by(|a, b| b.timestamp.cmp(&a.timestamp))
        .collect();

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
                    (toki_lili())
                    (feed(feed_props))
                }
                (trending())
            }
        },
        "Toki Lili",
    )
}
