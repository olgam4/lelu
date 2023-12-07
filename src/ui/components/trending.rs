use maud::{Markup, html};

pub fn trending() -> Markup {
    html! {
        div {
            form {
                div class="relative" {
                    input type="text" placeholder="alasa" class="bg-gray-100 p-4 rounded-lg w-full relative" {}
                    button class="i-carbon-search w-4 h-4 absolute top-1/2 right-4 transform -translate-y-1/2 text-gray-400" {}
                }
            }
            div class="bg-gray-100 p-4 rounded-lg mt-4" {
                h2 { "epiku" }
                ul {
                    li { a href="/hashtag/pona" { "#pona" } }
                    li { a href="/hashtag/toki" { "#toki" } }
                    li { a href="/hashtag/lili" { "#lili" } }
                }
            }
        }
    }
}


