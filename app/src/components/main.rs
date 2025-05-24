use dioxus::prelude::*;

#[component]
pub fn Main() -> Element {
    rsx!(
        div {
            id: "mainContainer",
            class: "MyContainer",
            h1 { "w" span { style: "color: #1e6da5;", "AI" } "ves" }
            h3 { "Want to surf the perfect wave ?" }
            h3 { "Don't let your surf session turn into a remake of " em {"Brice de Nice"} " !" }
            h3 { "Use wAIves to predict wave height 🌊🏄‍♂️" }
            form {
                id: "mainForm",
                label { "Select a model :" }
                br {}
                br {}
                select {
                    id: "modelMain",
                    option {
                        value: "wAIves1v5.0",
                        selected: true,
                        "wAIves1 v5.0"
                    },
                    option {
                        value: "wAIves1v5.1",
                        "wAIves1 v5.1"
                    },
                    option {
                        value: "wAIves2v1.0",
                        "wAIves2 v1.0"
                    }
                }
                br {}
            }
            br {}
            h4 { "Enter the location where you want to surf :" }
            div {
                class: "autocomplete-container",
                input {
                    id: "countryInput",
                    placeholder: "Country",
                    type: "text",

                }
                div {
                    id: "autocomplete-results",
                    class: "autocomplete-results"
                }
                input {
                    id: "cityInput",
                    placeholder: "City",
                    type: "text"
                }
            }
            br {}
            button {
                id: "getPrediction",
                onclick: move |_| {
                    todo!("Not yet implemented")
                },
                "Wave height prediction"
            }
            br {}
            h3 {
                id: "resultat",
            }
        }
    )
}