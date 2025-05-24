use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx! {
        div {
            id: "header",
            class: "menu",
            a { href: "#mainContainer", img { src: asset!("./assets/img/wAIves.png") } }
            a { href: "#playgroundContainer", img { src: asset!("./assets/img/playground.png") } }
            a { href: "https://github.com/LugolBis/wAIves", img { src: asset!("./assets/img/github.png") } }
        }
    }
}