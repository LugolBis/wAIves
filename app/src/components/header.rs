use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx! {
        div {
            id: "header",
            class: "menu",
            a { href: "#mainContainer", img { src: "https://cdn.jsdelivr.net/gh/LugolBis/wAIves@main/app/assets/img/wAIves.png" } }
            a { href: "#playgroundContainer", img { src: "https://cdn.jsdelivr.net/gh/LugolBis/wAIves@main/app/assets/img/playground.png" } }
            a { href: "https://github.com/LugolBis/wAIves", img { src: "https://cdn.jsdelivr.net/gh/LugolBis/wAIves@main/app/assets/img/github.png" } }
        }
    }
}
