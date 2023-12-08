use maud::{html, Markup};

use crate::domain::{Lili, Profile};

pub fn lili(lili: Lili, poster: Profile) -> Markup {
    html! {
        div
            class="bg-gray-100 p-4 rounded-lg" {
            div
                class="flex space-x-4"
                x-data={(format!("{{ date: new Date({}) }}", lili.timestamp * 1000))} {
                a href=(format!("/profile/{}", poster.username)) hx-boost="true" {
                    img width="0" src=(poster.avatar) class="w-10 h-10 rounded-full" alt="avatar" {}
                }
                div class="flex-1" {
                    div class="flex justify-between items-center" {
                        a href=(format!("/profile/{}", poster.username)) hx-boost="true" {
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
                        @if lili.liked_by_me {
                            (pona_active(lili.id, lili.ponas))
                        } @else {
                            (pona_inactive(lili.id, lili.ponas))
                        }
                    }
                    li {
                        button class="bg-gray-100 text-gray-400 p-2 rounded-full cursor-pointer hover:text-blue-500 transition-colors" {
                            div class="i-carbon-repeat w-4 h-4 inline-block mr-1" {}
                        }
                    }
                    li {
                        button
                            class="bg-gray-100 text-gray-400 p-2 rounded-full cursor-pointer hover:text-blue-500 transition-colors" {
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

pub fn pona_active(lili_id: String, qty: i64) -> Markup {
    html! {
        button
            hx-post=(format!("/ike/{}", lili_id))
            hx-swap="outerHTML"
            class="bg-gray-100 text-red-400 p-2 rounded-full cursor-pointer hover:text-blue-500 transition-colors" {
            div class="w-4 h-4 inline-block mr-1" { "pona" }
        }
        span class="text-red-400" { (qty) }
    }
}

pub fn pona_inactive(lili_id: String, qty: i64) -> Markup {
    html! {
        button
            hx-post=(format!("/pona/{}", lili_id))
            hx-swap="outerHTML"
            class="bg-gray-100 text-gray-400 p-2 rounded-full cursor-pointer hover:text-blue-500 transition-colors" {
            div class="w-4 h-4 inline-block mr-1" { "pona" }
        }
        span class="text-red-400" { (qty) }
    }
}
