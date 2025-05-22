//! This module implement a solution to perform a clustering on the coordinates data <br>
//! **Note** : The function *kmeans()* use parallelisme, to better performance feel free to use crossbeam

use std::collections::HashMap;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};
use std::thread;
use std::io::{Write};
use std::str::FromStr;
use std::fs::{self, OpenOptions};
use std::path::PathBuf;
use std::thread::JoinHandle;
use serde_json::{Value, to_string};

use crate::logs::*;
use crate::coordinates::custom_round;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Coordinate(f64,f64);

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
struct Distance(f64);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Region {
    UE,
    AF,
    TH,
    LAKE,
    UsaEst,
    UsaWest,
    UsaNorth,
    GolfMexico,
    CARIBBEAN
}

#[derive(Debug, Eq, Hash, Clone, Copy)]
struct Centroid(Region, Coordinate);

impl Coordinate {
    fn set(&mut self, x:f64, y:f64) {
        (self.0, self.1) = (x, y)
    }

    fn add(&mut self, x:f64, y:f64) {
        (self.0, self.1) = (self.0+x, self.1+y)
    }

    fn unwrap_vec(coordinates: &Vec<Coordinate>) -> Vec<f64> {
        let mut vector: Vec<f64> = Vec::new();
        for coordinate in coordinates {
            vector.push(coordinate.0);
            vector.push(coordinate.1);
        }
        vector
    }
}

impl Hash for Coordinate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
        self.1.to_bits().hash(state);
    }
}

impl Eq for Coordinate {}

impl Eq for Distance {}

impl Ord for Distance {
    fn cmp(&self, other: &Self) -> Ordering {
        let (x, y) = ((self.0*1000.0) as i64, (other.0*1000.0) as i64);
        if x == y { Ordering::Equal }
        else if x<y { Ordering::Less }
        else { Ordering::Greater }
    }
}

impl PartialEq for Centroid {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Centroid {
    fn get_coordinate(&self) -> &Coordinate {
        &self.1
    }

    fn set_coordinate(&mut self, new_coordinate: Coordinate) {
        self.1 = new_coordinate
    }

    fn update(&mut self, vector: &Vec<Coordinate>) {
        if vector.is_empty() {
            return;
        }

        let mut new_coordinate = Coordinate(0f64, 0f64);
        for coordinate in vector {
            new_coordinate.add(coordinate.0, coordinate.1)
        }

        new_coordinate.set(
            custom_round(new_coordinate.0/(vector.len() as f64)),
            custom_round(new_coordinate.1/(vector.len() as f64)),
        );

        self.set_coordinate(new_coordinate);
    }
}

const INITIAL_CENTROIDS: [Centroid;9] = [
    Centroid(Region::UE,Coordinate(36.0,25.0)), Centroid(Region::AF,Coordinate(22.0,38.0)), Centroid(Region::TH,Coordinate(20.0,-157.0)),
    Centroid(Region::LAKE,Coordinate(44.0,-83.5)), Centroid(Region::UsaEst,Coordinate(37.0, -67.0)),
    Centroid(Region::GolfMexico,Coordinate(29.0, -89.0)), Centroid(Region::CARIBBEAN,Coordinate(18.0, -65.0)),
    Centroid(Region::UsaWest,Coordinate(39.0, 123.0)), Centroid(Region::UsaNorth,Coordinate(60.0, -148.0))
];

pub fn clustering(file_path: &str, save_path: &str) {
    let file_path = PathBuf::from(file_path);

    if let Ok(content) = fs::read_to_string(&file_path) {
        if let Ok(json_object) = Value::from_str(&content) {
            match json_object {
                Value::Array(vector) => {
                    let mut coordinates: Vec<Coordinate> =  Vec::new();
                    let (mut index_lon, mut index_lat) = (0usize, 1usize);
                    while let (Some(longitude), Some(latitude)) = (vector.get(index_lon), vector.get(index_lat)) {
                        match (longitude, latitude) {
                            (Value::Number(x), Value::Number(y)) => {
                                if let (Some(x), Some(y)) = (x.as_f64(), y.as_f64()) {
                                    //let (x,y) = (custom_round(x), custom_round(y));
                                    coordinates.push(Coordinate(x,y));
                                }
                                else { log(format!("\nERROR : ({:?}, {:?}) can't be converted as f64 : {}",x,y,file_path.display())) }
                            },
                            _ => { log(format!("\nERROR : longitude and latitude deserialized Value are not Number : {}",file_path.display())) }
                        }
                        (index_lon, index_lat) = (index_lon+7, index_lat+7);
                    }

                    if let Ok((clusters, centroids)) = kmeans(coordinates) {
                        let save_path = PathBuf::from(save_path);
                        dump_json(&save_path, &clusters);

                        let save_path = PathBuf::from("centroids.json");
                        dump_json_centroid(&save_path, &centroids);
                    }
                    else { log(format!("\nERROR : {} the kmeans failed.",file_path.display())) }
                },
                _ => { log(format!("\nERROR : {} inconsistant value, it's not the format expected", file_path.display())) }
            }
        }
        else { log(format!("\nERROR : {} when try to convert the content into a Value", file_path.display())) }
    }
    else { log(format!("\nERROR : {} when try to read the content", file_path.display())) }
}

fn kmeans(vector: Vec<Coordinate>) -> Result<(Vec<Vec<f64>>, Vec<String>), ()> {
    let mut hashmap: HashMap<Centroid, Vec<Coordinate>> = HashMap::new();
    for centroid in INITIAL_CENTROIDS {
        hashmap.insert(centroid, vec![]);
    }

    let hashmap_arc = Arc::new(Mutex::new(hashmap));
    let vector_arc = Arc::new(vector);

    let num_threads = thread::available_parallelism().map_err(|_|())?.get();
    let chunk_size = (vector_arc.len() + num_threads - 1) / num_threads;
    
    for _ in 0..100 {
        let mut handles: Vec<JoinHandle<()>> = Vec::new();
        let centroids = hashmap_arc.lock().unwrap().keys().into_iter().map(|x| x.clone()).collect::<Vec<Centroid>>();
        let centroids_arc = Arc::new(centroids);

        for thread_number in 0..num_threads {
            let h_arc = Arc::clone(&hashmap_arc);
            let c_arc = Arc::clone(&centroids_arc);
            let v_arc = Arc::clone(&vector_arc);
            let start = thread_number * chunk_size;
            let end = (start + chunk_size).min(v_arc.len());

            let handle = thread::spawn(move || {
                for index in start..end {
                    if let Some(coordinate) = v_arc.get(index) {
                        let mut distances: Vec<Distance> = Vec::new();
                        let local_centroids = c_arc.as_slice();
                        for centroid in local_centroids {
                            distances.push(manhattan_distance(*coordinate, *centroid.get_coordinate()))
                        }

                        if let Some(index_distance) = distance_min(&distances) {
                            let choosed_centroid = local_centroids[index_distance];
                            if let Some(value) = h_arc.lock().unwrap().get_mut(&choosed_centroid) {
                                if !value.contains(&coordinate) {
                                    value.push(coordinate.clone());
                                }
                            }
                            else { log(format!("\nERROR : {} can't get the coordinate from {:?}",thread_number,h_arc)); }
                        }
                        else { log(format!("\nERROR : {} can't get the distance min index from {:?}",thread_number,distances)); }
                    }
                    else { log(format!("\nERROR : {} can't get the coordinate from {:?}",thread_number,v_arc)); }
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let mut new_centroids: Vec<Centroid> = Vec::new();
        let mut curent_coordinates: Vec<Vec<Coordinate>> = Vec::new();
        for centroid in centroids_arc.as_slice() {
            let mut centroid = *centroid;
            if let Some(coordinates) = hashmap_arc.lock().unwrap().remove(&centroid) {
                if !coordinates.is_empty() {
                    centroid.update(&coordinates);
                    curent_coordinates.push(coordinates);
                    new_centroids.push(centroid);
                }
            }
        }
        for (key, value) in new_centroids.iter().zip(curent_coordinates) {
            hashmap_arc.lock().unwrap().insert(*key, value);
        }
    }

    let mut clusters: Vec<Vec<f64>> = Vec::new();
    let mut centroids: Vec<String> = Vec::new();
    for (key, value) in hashmap_arc.lock().unwrap().iter() {
        clusters.push(Coordinate::unwrap_vec(value));
        centroids.push(format!("{:?};{};{}",key.0,key.1.0,key.1.1))
    }

    Ok((clusters,centroids))
}

fn manhattan_distance(coord0: Coordinate, coord1: Coordinate) -> Distance {
    Distance((coord0.0 - coord1.0).abs() + (coord0.1 - coord1.1).abs())
}

fn distance_min(distances: &Vec<Distance>) -> Option<usize> {
    distances.iter().enumerate().min_by_key(|(_, valeur)| *valeur).map(|(index, _)| index)
}

fn dump_json(file_path: &PathBuf, vector: &Vec<Vec<f64>>) {
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

fn dump_json_centroid(file_path: &PathBuf, vector: &Vec<String>) {
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