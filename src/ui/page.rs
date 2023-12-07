use maud::{html, Markup, PreEscaped};

use crate::infra::MaudTemplate;

fn htmx() -> Markup {
    html! {
        script src="https://unpkg.com/htmx.org@1.9.9" integrity="sha384-QFjmbokDn2DjBjq+fM+8LUIVrAgqcNW2s0PjAxHETgRn9l4fvX31ZxDxvwQnyMOX" crossorigin="anonymous" {}
        script defer src="/static/notify.js" {}
        script src="https://unpkg.com/htmx.org/dist/ext/preload.js" {}
    }
}

fn alpine() -> Markup {
    html! {
        script defer src="https://cdn.jsdelivr.net/npm/@marcreichel/alpine-timeago@latest/dist/alpine-timeago.min.js" {}
        script defer src="https://cdn.jsdelivr.net/npm/alpinejs@3.x.x/dist/cdn.min.js" {}
    }
}

fn uno() -> Markup {
    html! {
        link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@unocss/reset/tailwind-compat.min.css" {}
        script src="https://cdn.jsdelivr.net/npm/@unocss/runtime/uno.global.js" {}
        script src="https://cdn.jsdelivr.net/npm/@unocss/runtime/preset-icons.global.js" {}
        script { (PreEscaped("
          window.__unocss = {
            presets: [
              () => window.__unocss_runtime.presets.presetIcons({
                scale: 1.2,
                cdn: 'https://esm.sh/'
              }),
            ],
          }
        ")) }
        script src="https://cdn.jsdelivr.net/npm/@unocss/runtime/core.global.js" {}
    }
}

fn toki_pona() -> Markup {
    html! {
        style { "
            [un-cloak] {
                display: none !important;
            }

            @font-face {
                font-family: 'sitelen pona';
                src: url('/static/linja-pona-4.9.otf') format('truetype');
            }

            .sitelen-pona {
                font-family: 'sitelen pona';
            }

            .normal {
                font-family: 'Inter', sans-serif;
            }
        " }
    }
}

pub fn page(children: Markup, title: &str) -> MaudTemplate {
    html! {
        head {
            meta charset="utf-8" {}
            meta http-equiv="X-UA-Compatible" content="IE=edge" {}
            meta name="view-transition" content="same-origin" {}
            meta name="viewport" content="width=device-width, initial-scale=1.0" {}

            title { (title) }
            link rel="icon" href="/static/favicon.ico" type="image/x-icon" {}
            link rel="stylesheet" href="/static/main.css" {}

            script src="/static/hash.js" {}

            link rel="stylesheet" type="text/css" href="https://cdn.jsdelivr.net/npm/toastify-js/src/toastify.min.css" {}
            script type="text/javascript" src="https://cdn.jsdelivr.net/npm/toastify-js" {}

            (htmx())
            (alpine())
            (uno())
            (toki_pona())
        }
        body hx-ext="preload" class="sitelen-pona" un-cloak {
            (children)
            script defer { (PreEscaped("
                htmx.config.globalViewTransitions = true;
            ")) }
        }
    }
    .into()
}
