use maud::{html, Markup};

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
                label for="text" class="sr-only" { "toki lili" }
                textarea
                    id="text"
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
