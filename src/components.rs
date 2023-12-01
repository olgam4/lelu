use maud::{html, Markup};
use serde::{Deserialize, Serialize};

use crate::Lili;

pub fn login_form() -> Markup {
    html! {
        div class="m-4" {
            form
                hx-post="/login"
                x-data
                x-on:submit="setTimeout(() => {
                        $el.reset();
                        $el.querySelector('[name=username]').classList.remove('normal');
                        $el.querySelector('[name=password]').classList.remove('normal');
                    }, 1)"
                "hx-on::config-request"="event.detail.parameters['password'] = window.hash(event.detail.parameters['password'])"
                class="flex flex-col space-y-4" {
                input
                    name="username"
                    class="bg-gray-100 p-4 rounded-lg w-full"
                    placeholder="nimi+mi"
                    x-on:change="if ($el.value) $el.classList.add('normal'); else $el.classList.remove('normal')"
                    type="text" {}
                input
                    name="password"
                    class="bg-gray-100 p-4 rounded-lg w-full"
                    placeholder="nimi+awen"
                    x-on:change="if ($el.value) $el.classList.add('normal'); else $el.classList.remove('normal')"
                    type="password" {}
                input
                    type="submit"
                    class="bg-blue-500 w-30 text-white p-4 rounded-lg cursor-pointer hover:bg-blue-600 transition-colors self-end"
                    value="sitelen" {}
            }
        }
    }
}

pub fn sign_up_form() -> Markup {
    html! {
        div class="m-4" {
            form
                hx-post="/signup"
                x-data
                x-on:submit="setTimeout(() => {
                        $el.reset();
                        $el.querySelector('[name=username]').classList.remove('normal');
                        $el.querySelector('[name=password]').classList.remove('normal');
                    }, 1)"
                "hx-on::config-request"="event.detail.parameters['password'] = window.hash(event.detail.parameters['password'])"
                class="flex flex-col space-y-4" {
                input
                    name="username"
                    class="bg-gray-100 p-4 rounded-lg w-full"
                    placeholder="nimi+mi"
                    x-on:change="if ($el.value) $el.classList.add('normal'); else $el.classList.remove('normal')"
                    type="text" {}
                input
                    name="password"
                    class="bg-gray-100 p-4 rounded-lg w-full"
                    placeholder="nimi+awen"
                    x-on:change="if ($el.value) $el.classList.add('normal'); else $el.classList.remove('normal')"
                    type="password" {}
                input
                    type="submit"
                    class="bg-blue-500 w-30 text-white p-4 rounded-lg cursor-pointer hover:bg-blue-600 transition-colors self-end"
                    value="sitelen" {}
            }
        }
    }
}

pub fn lili(lili: Lili, poster: Profile) -> Markup {
    html! {
        div
            class="bg-gray-100 p-4 rounded-lg" {
            div
                class="flex space-x-4"
                x-data={(format!("{{ date: new Date({}) }}", lili.timestamp * 1000))} {
                img src=(poster.avatar) class="w-10 h-10 rounded-full" alt="avatar" {}
                div class="flex-1" {
                    div class="flex justify-between items-center" {
                        div {
                            h2 class="text-lg" {
                                (poster.name)
                                span class="text-gray-400 text-sm normal" { "@" (poster.username) }
                            }
                        }
                        div class="text-gray-400 normal" x-timeago="date" {}
                    }
                    p {
                        @for word in lili.text.split_whitespace() {
                            @if word.starts_with("#") {
                                span class="text-blue-500" { (word) }
                            } @else {
                                (word)
                            }
                            " "
                        }
                    }
                }
            }
            div class="flex" {
                ul class="flex justify-between mx-auto w-3/4" {
                    li {
                        button class="bg-gray-100 text-gray-400 p-2 rounded-full cursor-pointer hover:text-blue-500 transition-colors" {
                            div class="i-carbon-chat w-4 h-4 inline-block mr-1" {}
                        }
                    }
                    li {
                        button class="bg-gray-100 text-gray-400 p-2 rounded-full cursor-pointer hover:text-red-500 transition-colors" {
                            div class="w-4 h-4 inline-block mr-1" { "pona" }
                        }
                    }
                    li {
                        button class="bg-gray-100 text-gray-400 p-2 rounded-full cursor-pointer hover:text-blue-500 transition-colors" {
                            div class="i-carbon-repeat w-4 h-4 inline-block mr-1" {}
                        }
                    }
                    li {
                        button class="bg-gray-100 text-gray-400 p-2 rounded-full cursor-pointer hover:text-blue-500 transition-colors" {
                            div class="i-carbon-bookmark w-4 h-4 inline-block mr-1" {}
                        }
                    }
                    li {
                        button class="bg-gray-100 text-gray-400 p-2 rounded-full cursor-pointer hover:text-blue-500 transition-colors" {
                            div class="i-carbon-share w-4 h-4 inline-block mr-1" {}
                        }
                    }
                }
            }
        }
    }
}

pub fn nav_link(href: &str, text: &str) -> Markup {
    html! {
        a href=(href) class="text-blue-500 hover:text-blue-600 transition-colors" preload="mouseover" { (text) }
    }
}

pub fn nav() -> Markup {
    html! {
        nav {
            ul {
                li { (nav_link("/", "toki+lili")) }
                li { (nav_link("/profile", "mi")) }
                li { (nav_link("/signup", "pali e lipu")) }
            }
        }
    }
}

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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Profile {
    pub username: String,
    pub name: String,
    pub avatar: String,
    pub bio: String,
    pub website: String,
    pub location: String,
    pub birthday: String,
}

pub fn profile(a_profile: Profile) -> Markup {
    html! {
        div {
            div class="flex space-y-4 flex-col" {
                div class="border-b space-y-4 pb-4" {
                    div class="flex justify-between items-center" {
                        div {
                            img src=(a_profile.avatar) class="w-20 h-20 rounded-full" alt="avatar" {}
                        }
                        div {
                            ul class="flex space-x-4" {
                                li {
                                    button class="bg-gray-100 text-gray-400 p-2 w-[40px] h-[40px] rounded-full cursor-pointer hover:text-blue-500 transition-colors" {
                                        "toki"
                                    }
                                }
                                li {
                                    button class="bg-blue-400 text-white p-2 w-[40px] h-[40px] rounded-full cursor-pointer hover:text-blue-500 transition-colors" {
                                        "kama+sina"
                                    }
                                }
                            }
                        }
                    }
                    h2 class="text-2xl" {
                        (a_profile.name)
                        span class="text-gray-400 text-sm normal" { "@" (a_profile.username) }
                    }
                    p class="text-gray-400" { (a_profile.bio) }
                    div class="flex space-x-4" {
                        span {
                            div class="i-carbon-location w-4 h-4 inline-block mr-1" {}
                            (a_profile.location)
                        }
                        span {
                            div class="i-carbon-earth-southeast-asia w-4 h-4 inline-block mr-1" {}
                            a href=(a_profile.website) class="text-blue-500 hover:text-blue-600 transition-colors normal" { (a_profile.website) }
                        }
                        span {
                            div class="i-ic-outline-cake w-4 h-4 inline-block mr-1" {}
                            (a_profile.birthday)
                        }
                    }
                }
                div class="" {
                }
            }
        }
    }
}

pub fn toki_lili() -> Markup {
    html! {
        form
            hx-post="/toki_lili"
            hx-target="#feed"
            hx-swap="afterbegin"
            class="flex flex-col space-y-4"
            x-on:submit="setTimeout(() => $el.reset(), 1)"
            x-data="{ text: '' }" {
            div class="relative flex" {
                textarea
                    name="text"
                    x-model="text"
                    class="bg-gray-100 p-4 rounded-lg w-full resize-none normal"
                    maxlength="280" {}
                p class="absolute bottom-0 right-4 text-gray-400 pointer-events-none" {
                    span x-text="text.length" {}
                    " / 280"
                }
            }
            input
                type="submit"
                class="bg-blue-500 w-30 text-white py-2 rounded-lg cursor-pointer hover:bg-blue-600 transition-colors self-end disabled:bg-gray-400"
                "x-bind:disabled"="text.length === 0"
                value="toki+lili" {}
        }
    }
}

pub fn toast(text: String) -> Markup {
    html! {
        div
            class="bg-green-500 text-white p-4 rounded-lg"
            x-data
            x-init="setTimeout(() => $el.remove(), 3000)"
        {
            p { (text) }
        }
    }
}
