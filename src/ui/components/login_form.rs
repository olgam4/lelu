use maud::{Markup, html};

pub fn login_form() -> Markup {
    html! {
        div class="m-4" {
            form
                hx-post="/login"
                hx-target="body"
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
