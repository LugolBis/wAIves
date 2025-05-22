//! The aim of this module is to provide a solution to extract the coordinates from the dataset

use std::collections::HashSet;
use std::fs::OpenOptions;
use std::io::{Write};
use std::str::FromStr;
use std::fs::{self, DirEntry};
use std::path::PathBuf;
use serde_json::{Value, to_string};

use crate::logs::*;

pub fn extract_coordinates(folder_path: &str) {
    let mut coordinates: HashSet<(String,String)> = HashSet::new();
    let mut result: Vec<f64> = Vec::new();
    let save_path: PathBuf = PathBuf::from("coordinates.json");

    match fs::read_dir(folder_path) {
        Ok(entries) => {
            let entries = entries
                .filter(|e| e.is_ok())
                .map(|x| x.unwrap())
                .collect::<Vec<DirEntry>>();

            for entry in entries {
                let file_path = entry.path();
                parse_file(&mut coordinates, file_path);
            }
        },
        Err(error) => log(format!("\nERROR : extract_coordinates() : {}",error)),
    }

    for (longitude, latitude) in coordinates {
        if let (Ok(x), Ok(y)) = (longitude.parse::<f64>(), latitude.parse::<f64>()) {
            result.push(x);
            result.push(y);
        }
        else {
            log(format!("\nERROR : can't parse to f64 : ({},{})",longitude, latitude));
        }
    }

    dump_json(&save_path, &result);
}

fn parse_file(coordinates: &mut HashSet<(String,String)>, file_path: PathBuf) {
    let mut local_coordinates: HashSet<(String,String)> = HashSet::new();

    if let Ok(content) = fs::read_to_string(&file_path) {
        if let Ok(json_object) = Value::from_str(&content) {
            match json_object {
                Value::Array(vector) => {
                    let (mut index_lon, mut index_lat) = (0usize, 1usize);
                    while let (Some(longitude), Some(latitude)) = (vector.get(index_lon), vector.get(index_lat)) {
                        match (longitude, latitude) {
                            (Value::Number(x), Value::Number(y)) => {
                                if let (Some(x), Some(y)) = (x.as_f64(), y.as_f64()) {
                                    let (x, y) = (format!("{x}"), format!("{y}"));
                                    local_coordinates.insert((x,y));
                                }
                                else { log(format!("\nERROR : ({:?}, {:?}) can't be converted as f64 : {}",x,y,file_path.display())) }
                            },
                            _ => { log(format!("\nERROR : longitude and latitude deserialized Value are not Number : {}",file_path.display())) }
                        }
                        (index_lon, index_lat) = (index_lon+7, index_lat+7);
                    }

                    for coords in local_coordinates {
                        coordinates.insert(coords);
                    }
                },
                _ => { log(format!("\nERROR : the json object hasn't the expected type : {}",file_path.display())); }
            }
        }
        else { log(format!("\nERROR : can't deserialize the file : {}",file_path.display())) }
    }
    else { log(format!("\nERROR : can't open the file : {}",file_path.display())) }
}

#[allow(unused)]
const OUTLIERS_STATIONS: [(f64,f64);12] = [
    (13.289, 20.0), (10.051, -125.032), (13.729, -144.668), (0.0, -153.913), (7.079, 171.384),
    (15.268, 145.662), (13.683, 144.816), (13.354, 144.788), (59.94, -39.52), (7.081,158.244),
    (7.63,134.671), (-14.273,-170.501)
];

#[allow(unused)]
pub fn filter_outlier(file_path: &str) {
    let file_path = PathBuf::from(file_path);
    let mut filtered_data: Vec<f64> = Vec::new();

    let interval = |x:f64,y:f64| {
        30.000 <= x && x <= 50.000 && -74.000 <= y && y <= -60.000 
    };

    if let Ok(content) = fs::read_to_string(&file_path) {
        if let Ok(json_object) = Value::from_str(&content) {
            match json_object {
                Value::Array(vector) => {
                    let n = vector.len()-1;
                    for index in (0..n).step_by(2) {
                        let (longitude, latitude) = (&vector[index], &vector[index+1]);
                        if let (Some(x), Some(y)) = (longitude.as_f64(), latitude.as_f64()) {
                            let (x, y) = (custom_round(x), custom_round(y));
                            if !OUTLIERS_STATIONS.contains(&(y,x)) && !interval(x,y) {
                                filtered_data.push(x);
                                filtered_data.push(y);
                            }
                            else {
                                log(format!("\nSuccessfully filter the station : ({},{})",x,y))
                            }
                        }
                    }

                    dump_json(&file_path, &filtered_data);
                },
                _ => {
                    log(format!("\nERROR {} - the JSON object doesn't have the expected format.", dbg!(file_path.display())))
                }
            }
        }
        else { log(format!("\nERROR {} - can't deserialize the content.", dbg!(file_path.display()))) }
    }
    else { log(format!("\nERROR {} - can't read the file.", dbg!(file_path.display()))) }
}

pub fn custom_round(value: f64) -> f64 {
    (value * 1000.0).round() / 1000.0
}

fn dump_json(file_path: &PathBuf, vector: &Vec<f64>) {
    if let Ok(content) = to_string(&vector) {
        match OpenOptions::new().create(true).truncate(true).write(true).open(&file_path) {
            Ok(mut file) => {
                let _ = file.write_all(content.as_bytes());
            }
            Err(error) => { log(format!("\nERROR : {} in the following file : {}",error,file_path.display())); }
        }
    }
    else { log(format!("\nERROR : can't convert serialize the vector - {}",file_path.display())); }
}