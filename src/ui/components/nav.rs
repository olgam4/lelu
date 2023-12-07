use maud::{html, Markup};

pub struct NavProps {
    pub is_logged_in: bool,
}

pub fn nav_link(href: &str, text: &str) -> Markup {
    html! {
        a href=(href) hx-boost="true" class="text-blue-500 hover:text-blue-600 transition-colors" preload="mouseover" { (text) }
    }
}

pub fn nav(props: NavProps) -> Markup {
    html! {
        nav {
            ul {
                li { (nav_link("/", "toki+lili")) }
                @if props.is_logged_in {
                    li { (nav_link("/me", "mi")) }
                    li { (nav_link("/logout", "o pini")) }
                }
                @if !props.is_logged_in {
                    li { (nav_link("/login", "o pali e lipu")) }
                    li { (nav_link("/signup", "pali e lipu")) }
                }
            }
        }
    }
}
