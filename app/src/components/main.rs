use dioxus::prelude::*;
use crate::utils::*;
use web_sys::console;
use crate::data::*;

#[component]
pub fn Main() -> Element {
    let mut model = use_signal(|| "wAIves1v5.0".to_string());
    let mut country = use_signal(|| "".to_string());
    let mut city = use_signal(|| "".to_string());
    let mut loading = use_signal(|| false);
    let mut result = use_signal(|| "".to_string());

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
                    onchange: move |e| {
                        model.set(e.value().clone());
                    },
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
            Location { country: country, city: city }
            br {}
            button {
                id: "getPrediction",
                onclick: move |_| {
                    if !*loading.read() {
                        spawn({
                            async move {
                                loading.set(true);
                                if let Some(location) = get_location(&*city.read(), &*country.read()) {
                                    match get_weather_data(location.as_str()).await {
                                        Ok(data) => {
                                            match predict_with_model(data, &*model.read()).await {
                                                Ok(output) => {
                                                    result.set(format!("{output}"))
                                                },
                                                Err(error) => {
                                                    result.set("Error : Can't use the model. Please try again later.".to_string());
                                                    console::log_2(&"ERROR : when try to use the model : ".into(), &error.into())
                                                }
                                            }
                                        },
                                        Err(error) => {
                                            result.set("Error : Can't connect to the Weather API. Please use the 'Playground'.".to_string());
                                            console::log_2(&"ERROR : when try to get the weather data : ".into(), &error.into())
                                        }
                                    }
                                }
                                else {
                                    result.set("Error : Unknow country.".to_string());
                                }
                                loading.set(false);
                            }
                        });
                    }
                },
                "Wave height prediction"
            }
            br {}
            if *loading.read() {
                h3 {
                    id: "resultat",
                    "Loading ..."
                }
            }
            else {
                if !result.read().is_empty() {
                    h3 {
                        id: "resultat",
                        "{&*result.read()} m"
                    }
                }
            }
        }
    )
}

#[component]
fn Location(country: Signal<String>, city: Signal<String>) -> Element {
    let mut results = use_signal(|| Vec::<String>::new());
    let mut show_results = use_signal(|| false);

    use_effect(move || {
        if country().is_empty() {
            results.set(Vec::new());
            show_results.set(false);
        } else {
            let filtered = COUNTRIES.iter()
                .filter(|ca| ca.0.to_lowercase().starts_with(&country().to_lowercase()))
                .map(|ca| String::from(ca.0))
                .collect();
            results.set(filtered);
        }
    });

    rsx!(
        div {
            class: "autocomplete-container",
            input {
                id: "countryInput",
                placeholder: "Country",
                r#type: "text",
                value: "{country}",
                oninput: move |e| {
                    country.set(e.value());
                    show_results.set(true);
                },
                onfocus: move |_| {
                    show_results.set(true)
                }
            }
            if show_results() && !results().is_empty() {
                div {
                    id: "autocomplete-results",
                    class: "autocomplete-results",
                    div { 
                        class: "result-item",
                        background_color: "#5a7d9a",
                        color: "#ffffff",
                        onclick: move |_| {
                            show_results.set(false);
                        },
                        "See less"
                    }
                    for result in results() {
                        div {
                            class: "result-item",
                            onclick: move |_| {
                                country.set(result.to_string());
                                show_results.set(false);
                            },
                            "{result}"
                        }
                    }
                }
            }
            input {
                id: "cityInput",
                placeholder: "City",
                r#type: "text",
                value: "{city}",
                oninput: move |e| city.set(e.value())
            }
        }
    )
}