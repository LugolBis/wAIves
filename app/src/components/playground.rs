use dioxus::prelude::*;
use crate::utils::*;
use web_sys::console;

#[component]
pub fn Playground() -> Element {
    let mut model = use_signal(|| "wAIves1v5.0".to_string());
    
    let mut longitude = use_signal(|| "".to_string());
    let mut latitude = use_signal(|| "".to_string());
    let mut temperature = use_signal(|| "".to_string());
    let mut pressure = use_signal(|| "".to_string());
    let mut wind_speed = use_signal(|| "".to_string());
    let mut wind_direction = use_signal(|| "".to_string());
    
    let mut loading = use_signal(|| false);
    let mut result = use_signal(|| "".to_string());

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
            div {
                id: "playgroundInputs",
                br {}
                input {
                    id: "inputLongitude",
                    placeholder: "Longitude (DD)",
                    value: "{longitude}",
                    r#type: "text",
                    oninput: move |e| { longitude.set(e.value()) }
                }
                input {
                    id: "inputLatitude",
                    placeholder: "Latitude (DD)",
                    value: "{latitude}",
                    r#type: "text",
                    oninput: move |e| { latitude.set(e.value()) }
                }
                br {}
                input {
                    id: "inputTemperature",
                    placeholder: "Temperature (°C)",
                    value: "{temperature}",
                    r#type: "text",
                    oninput: move |e| { temperature.set(e.value()) }
                }
                input {
                    id: "inputPressure",
                    placeholder: "Pressure (hPa)",
                    value: "{pressure}",
                    r#type: "text",
                    oninput: move |e| { pressure.set(e.value()) }
                }
                br {}
                input {
                    id: "inputWindSpeed",
                    placeholder: "Wind speed (m/s)",
                    value: "{wind_speed}",
                    r#type: "text",
                    oninput: move |e| { wind_speed.set(e.value()) }
                }
                input {
                    id: "inputWindDirection",
                    placeholder: "Wind direction (°)",
                    value: "{wind_direction}",
                    r#type: "text",
                    oninput: move |e| { wind_direction.set(e.value()) }
                }
            }
            br {}
            button {
                id: "getPlaygroundPrediction",
                onclick: move |_| {
                    if !*loading.read() {
                        spawn({
                            async move {
                                loading.set(true);
                                match parse_inputs(
                                    &*longitude.read(), &*latitude.read(), &*temperature.read(), &*pressure.read(), &*wind_speed.read(), &*wind_direction.read()
                                )
                                {
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
                                        result.set(format!("Error : {error}"));
                                    }
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
                    id: "resultatPlayground",
                    "Loading ..."
                }
            }
            else {
                if !result.read().is_empty() {
                    h3 {
                        id: "resultatPlayground",
                        "{&*result.read()} m"
                    }
                }
            }
        }
    )
}

fn parse_inputs(longitude: &String,latitude: &String,temperature: &String,pressure: &String,wind_speed: &String,wind_direction: &String) -> Result<[f64;6], String> {
    match (longitude.parse::<f64>(),latitude.parse::<f64>(),temperature.parse::<f64>(),pressure.parse::<f64>(),wind_speed.parse::<f64>(),wind_direction.parse::<f64>()) {
        (Ok(lon),Ok(lat),Ok(temp),Ok(press),Ok(winds),Ok(windd)) => {
            Ok([lon,lat,temp,press,winds,windd])
        },
        _ => {
            Err("Inconsistent input.".to_string())
        }
    }
}