use maud::html;

use crate::{
    infra::MaudTemplate,
    ui::{feed, nav, page, toki_lili, trending, FeedProps, NavProps},
};

pub struct HeroProps {
    pub feed_props: FeedProps,
    pub nav_props: NavProps,
}

pub fn hero(props: HeroProps) -> MaudTemplate {
    page(
        html! {
            div class="main-page" {
                (nav(props.nav_props))
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
