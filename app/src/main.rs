mod components;

use dioxus::prelude::*;
use crate::components::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/src/style.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Script { src: "https://cdn.jsdelivr.net/npm/@tensorflow/tfjs" }
        document::Script { src: "/src/data.js" }
        document::Script { src: "/src/main.js" }
        document::Script { src: "/src/playground.js" }

        Header {}
        Main {}
        Playground {}

        document::Script { src: "/src/event.js" }
    }
}
