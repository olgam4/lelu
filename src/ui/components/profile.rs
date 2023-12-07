use maud::{html, Markup};

use crate::domain::Profile;

pub fn profile(a_profile: Profile) -> Markup {
    html! {
        div {
            div class="flex space-y-4 flex-col" {
                div class="border-b space-y-4 pb-4" {
                    div class="flex justify-between items-center" {
                        div {
                            img width="0" src=(a_profile.avatar) class="w-20 h-20 rounded-full" alt="avatar" {}
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
