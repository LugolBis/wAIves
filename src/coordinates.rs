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

    if let Ok(content) = to_string(&result) {
        match OpenOptions::new().create(true).truncate(true).write(true).open(&save_path) {
            Ok(mut file) => {
                let _ = file.write_all(content.as_bytes());
            }
            Err(error) => { log(format!("\nERROR : extract_coordinates() : {} in the following file : {}",error,save_path.display())); }
        }
    }
    else {
        log(String::from("\nERROR : extract_coordinates() : can't convert result into string"));
    }
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
                            _ => { log(format!("\nERROR : longitude and latitude aren't deserialized Value are not Number : {}",file_path.display())) }
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