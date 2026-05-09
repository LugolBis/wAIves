mod components;
mod data;
mod utils;

use crate::components::*;
use dioxus::prelude::*;

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    use_effect(|| {
        if let Some(loader) = gloo::utils::document().get_element_by_id("wasm-loader-js") {
            loader.remove();
        }
    });

    rsx! {
        Header {}
        Main {}
        Playground {}
    }
}
