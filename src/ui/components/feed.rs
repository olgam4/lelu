use maud::{html, Markup};

use crate::{
    domain::{Lili, Profile},
    ui::components::lili,
};

pub struct FeedProps {
    pub lilis: Vec<(Lili, Profile)>,
}

pub fn feed(props: FeedProps) -> Markup {
    let lilis = props.lilis;

    html! {
        div id="feed" class="flex flex-col space-y-4" {
            @for current_lili in lilis {
                (lili(current_lili.0, current_lili.1))
            }
        }
    }
}
