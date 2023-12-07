use maud::html;

use crate::{infra::MaudTemplate, ui::{page, nav, toki_lili, feed, trending, FeedProps}};

pub struct HeroProps {
    pub feed_props: FeedProps,
}

pub fn hero(props: HeroProps) -> MaudTemplate {
    page(
        html! {
            div class="main-page" {
                (nav())
                div {
                    (toki_lili())
                    (feed(props.feed_props))
                }
                (trending())
            }
        },
        "Toki Lili",
    )
}
