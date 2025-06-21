mod components;
mod data;
mod utils;

use dioxus::prelude::*;
use crate::components::*;

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {

    rsx! {
        Header {}
        Main {}
        Playground {}
    }
}
