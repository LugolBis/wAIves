mod components;
mod data;
mod utils;

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

        Header {}
        Main {}
        Playground {}
    }
}
