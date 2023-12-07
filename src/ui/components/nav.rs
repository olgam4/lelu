use maud::{html, Markup};

pub fn nav_link(href: &str, text: &str) -> Markup {
    html! {
        a href=(href) hx-boost="true" class="text-blue-500 hover:text-blue-600 transition-colors" preload="mouseover" { (text) }
    }
}

pub fn nav() -> Markup {
    html! {
        nav {
            ul {
                li { (nav_link("/", "toki+lili")) }
                li { (nav_link("/me", "mi")) }
                li { (nav_link("/signup", "pali e lipu")) }
            }
        }
    }
}
