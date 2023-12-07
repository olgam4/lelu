use maud::html;

use crate::{
    domain::Profile,
    infra::MaudTemplate,
    ui::{feed, nav, page, profile, FeedProps, NavProps},
};

pub struct ProfileProps {
    pub feed_props: FeedProps,
    pub me_profile: Profile,
}

pub fn profile_page(props: ProfileProps) -> MaudTemplate {
    page(
        html! {
            div class="main-page" {
                (nav(NavProps { is_logged_in: true }))
                div {
                    (profile(props.me_profile))
                    (feed(props.feed_props))
                }
                div {}
            }
        },
        "Profile",
    )
}
