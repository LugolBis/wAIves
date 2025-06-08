use reqwest;
use crate::data::*;
use serde::Deserialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use js_sys::{Array, Promise};
use serde_json;

#[derive(Debug, Deserialize)]
pub struct WeatherData {
    coord: Coord,
    main: Main,
    wind: Wind,
}

#[derive(Debug, Deserialize)]
struct Coord {
    lon: f64,
    lat: f64,
}

#[derive(Debug, Deserialize)]
struct Main {
    temp: f64,
    pressure: f64,
}

#[derive(Debug, Deserialize)]
struct Wind {
    speed: f64,
    deg: f64,
}

#[derive(Debug, Clone, Copy)]
struct Distance(f64);

impl WeatherData {
    pub fn to_array(&self) -> [f64; 6] {
        [
            self.coord.lon,
            self.coord.lat,
            self.main.temp - 273.15,  // Convert Kelvin to Celsius
            self.main.pressure,
            self.wind.speed,
            self.wind.deg,
        ]
    }
}

pub fn get_location(city: &String, country: &String) -> Option<String> {
    for ca in COUNTRIES {
        if ca.0.contains(country) {
            return Some(format!("{}%2C{}",city,ca.1))
        } 
    }
    None
}

pub async fn get_weather_data(location: &str) -> Result<[f64; 6], String> {
    let url = format!("https://api.openweathermap.org/data/2.5/weather?q={}&appid=0a216b54f594e070778d7d8b8390ac06",location);
    
    let response = reqwest::Client::new()
        .get(url)
        .send()
        .await.map_err(|e| format!("{e}"))?
        .text()
        .await.map_err(|e| format!("{e}"))?;

    let weather_data: WeatherData = serde_json::from_str(&response)
        .map_err(|e| format!("{e}"))?;

    Ok(weather_data.to_array())
}

fn euclidean_distance(coord0: &Coordinate, coord1: &Coordinate) -> Distance {
    let (x, y) = (coord0.0-coord1.0, coord0.1-coord1.1);
    Distance((x*x + y*y).sqrt())
}

fn get_cluster(coordinate: Coordinate) -> usize {
    let mut distances: Vec<Distance> = Vec::new();

    for centroid in CENTROIDS {
        distances.push(euclidean_distance(&coordinate, &centroid.1));
    }

    let mut index = 0;
    for idx in 1..distances.len() {
        if distances[index].0 > distances[idx].0 {
            index = idx;
        }
    }

    index
}

fn get_url(data: &[f64], model: &str, truncate: &mut bool) -> String {
    match model {
        "wAIves1v5.0" => {
            "https://cdn.jsdelivr.net/gh/LugolBis/wAIves@rust/Models/wAIves1v5.0/model.json".to_string()
        },
        "wAIves1v5.1" => {
            "https://cdn.jsdelivr.net/gh/LugolBis/wAIves@rust/Models/wAIves1v5.1/model.json".to_string()
        },
        "wAIves2v1.0" => {
            *truncate = true;
            format!("https://cdn.jsdelivr.net/gh/LugolBis/wAIves@rust/Models/wAIves2v1.{}/model.json",get_cluster(Coordinate(data[0], data[1])))
        },
        _ => {
            "https://cdn.jsdelivr.net/gh/LugolBis/wAIves@rust/Models/wAIves1v5.0/model.json".to_string()
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = tfjsHelpers)]
    fn predict(input_array: &Array, model_name: &str, truncate: bool) -> Promise;
}

pub async fn predict_with_model(data: [f64;6], model: &str) -> Result<f64, String> {
    let js_array = Array::new();
    for value in data {
        js_array.push(&JsValue::from_f64(value));
    }

    let mut truncate = false;
    let url = get_url(&data, model, &mut truncate);

    let promise = predict(&js_array, &url, truncate);
    let js_result = JsFuture::from(promise)
        .await
        .map_err(|e| format!("Error JS: {:?}", e))?;

    js_result
        .as_f64()
        .ok_or_else(|| "Inconsistent result.".to_string())
}
