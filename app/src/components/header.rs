use dioxus::prelude::*;

const WAIVES: &str = "https://cdn.jsdelivr.net/gh/LugolBis/wAIves@main/app/assets/img/wAIves.png";
const XMARK: &str = "https://cdn.jsdelivr.net/gh/LugolBis/wAIves@main/app/assets/img/xmark.svg";
const MENU: &str = "https://cdn.jsdelivr.net/gh/LugolBis/wAIves@main/app/assets/img/menu.svg";
const PLAYGROUND: &str =
    "https://cdn.jsdelivr.net/gh/LugolBis/wAIves@main/app/assets/img/playground.png";
const GH: &str = "https://cdn.jsdelivr.net/gh/LugolBis/wAIves@main/app/assets/img/github.png";

#[component]
pub fn Header() -> Element {
    let mut menu_open = use_signal(|| false);

    rsx! {
        // Blur overlay
        if menu_open() {
            div { class: "menu-overlay", onclick: move |_| menu_open.set(false) }
        }

        div { id: "header", class: "menu",
            a { href: "#mainContainer", class: "menu-logo",
                img { src: WAIVES }
            }

            // Toggle button for small screens
            if !menu_open() {
                button {
                    class: "menu-toggle",
                    onclick: move |_| menu_open.set(true),
                    img { src: MENU, alt: "Open the menu" }
                }
            }

            div { class: if menu_open() { "menu-links menu-open" } else { "menu-links" },
                button {
                    class: "menu-close",
                    onclick: move |_| menu_open.set(false),
                    img { src: XMARK, alt: "Close the menu" }
                }
                a { href: "#playgroundContainer",
                    img { src: PLAYGROUND }
                }
                a { href: "https://github.com/LugolBis/wAIves",
                    img { src: GH }
                }
            }
        }
    }
}
