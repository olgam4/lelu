use maud::html;

use crate::{domain::Profile, ui::{FeedProps, page, feed, profile, nav}, infra::MaudTemplate};

pub struct ProfileProps {
    pub feed_props: FeedProps,
    pub me_profile: Profile,
}

pub fn profile_page(props: ProfileProps) -> MaudTemplate {
    page(
        html! {
            div class="main-page" {
                (nav())
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
