use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    let mut menu_open = use_signal(|| false);

    rsx! {
        div { id: "header", class: "menu",
            a { href: "#mainContainer", class: "menu-logo",
                img { src: "https://cdn.jsdelivr.net/gh/LugolBis/wAIves@main/app/assets/img/wAIves.png" }
            }

            // Toggle button for small screens
            button {
                class: "menu-toggle",
                onclick: move |_| menu_open.set(!menu_open()),
                if menu_open() {
                    "<"
                } else {
                    ">"
                }
            }

            div { class: if menu_open() { "menu-links menu-open" } else { "menu-links" },
                a { href: "#playgroundContainer",
                    img { src: "https://cdn.jsdelivr.net/gh/LugolBis/wAIves@main/app/assets/img/playground.png" }
                }
                a { href: "https://github.com/LugolBis/wAIves",
                    img { src: "https://cdn.jsdelivr.net/gh/LugolBis/wAIves@main/app/assets/img/github.png" }
                }
            }
        }
    }
}
