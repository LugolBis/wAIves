use dioxus::prelude::*;

#[component]
pub fn Playground() -> Element {
    let mut longitude: String =String::new();

    rsx!(
        div {
            id: "playgroundContainer",
            class: "MyContainer",
            br {}
            br {}
            h2 { "Playground" }
            h4 { "Test models with custom data" }
            h4 {
                "You can easily retrieve the data you need using the following website : "
                a {
                    href: "https://meteotrend.com/",
                    title: "Russian weather website",
                    target: "_blank",
                    "MeteoTrend"
                }
                " ."
            }
            form {
                id: "playgroundForm",
                label { "Select a model :" }
                br {}
                br {}
                select {
                    id: "modelPlayground",
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
            div {
                id: "playgroundInputs",
                br {}
                input {
                    id: "inputLongitude",
                    placeholder: "Longitude (DD)",
                    value: longitude.clone()
                }
                input {
                    id: "inputLatitude",
                    placeholder: "Latitude (DD)",
                    value: ""
                }
                br {}
                input {
                    id: "inputTemperature",
                    placeholder: "Temperature (°C)",
                    value: ""
                }
                input {
                    id: "inputPressure",
                    placeholder: "Pressure (hPa)",
                    value: ""
                }
                br {}
                input {
                    id: "inputWindSpeed",
                    placeholder: "Wind speed (m/s)",
                    value: ""
                }
                input {
                    id: "inputWindDirection",
                    placeholder: "Wind direction (°)",
                    value: ""
                }
            }
            br {}
            button {
                id: "getPredictionPlayground",
                onclick: move |_| {
                    println!("\n\nLongitude : {}\n\n",longitude);
                    todo!("Not yet implemented")
                },
                "Wave height prediction"
            }
            br {}
            h3 {
                id: "resultatPlayground"
            }
        }
    )
    
}