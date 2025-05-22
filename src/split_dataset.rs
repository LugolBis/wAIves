//! This module contains a function to split the curent dataset into multiples files group by the coordinates clusters

use std::fs::OpenOptions;
use std::io::Write;
use std::fs::{self, DirEntry};
use std::path::PathBuf;
use serde_json::{Value::{self,Number as NB}, to_string, from_str};
use std::str::FromStr;
use std::collections::HashMap;

use crate::logs::*;
use crate::coordinates::custom_round;

/// This function split the curent dataset into multiples files group by the coordinates clusters <br>
/// _cluster_coordinate_ is the path of the file that contains the coordinates of stations group by their cluster <br>
/// _data_folder_ is the path to the folder that contain the 
pub fn split_dataset(cluster_coordinate: &str, data_folder: &str, save_folder: &str) {
    if let Some(clusters) = get_clusters(PathBuf::from(cluster_coordinate)) {
        let mut result: HashMap<usize, Vec<f64>> = HashMap::new();
        for index in 0..clusters.len() {
            result.insert(index, Vec::new());
        }

        match fs::read_dir(data_folder) {
            Ok(entries) => {
                let entries = entries
                    .filter(|e| e.is_ok())
                    .map(|x| x.unwrap())
                    .collect::<Vec<DirEntry>>();
                
                let save_folder = PathBuf::from(save_folder);
                for entry in entries {
                    let file_path = entry.path();
                    split_file(&clusters, file_path, &mut result);
                }

                for (key, value) in result {
                    let path = save_folder.join(format!("dataC{}.json",key));
                    if let Ok(content) = to_string(&value) {
                        if let Ok(mut file) = OpenOptions::new().create(true).truncate(true).write(true).open(&path) {
                            let _ = file.write_all(content.as_bytes());
                        }
                        else { log(format!("\nERROR : when try to split the dataset :can't write in the following file : {}", path.display())); }
                    }
                    else { log(format!("\nERROR : when try to split the dataset : can't convert result into string {}",path.display())); }
                }
            },
            Err(error) => log(format!("\nERROR : seadatanet2() : {}",error)),
        }
    }
}

fn get_clusters(file_path: PathBuf) -> Option<Vec<Vec<f64>>> {
    let mut result: Vec<Vec<f64>> = Vec::new();

    if let Ok(content) = fs::read_to_string(&file_path) {
        if let Ok(json_object) = Value::from_str(&content) {
            match json_object {
                Value::Array(vec1) => {
                    for val in vec1 {
                        match val {
                            Value::Array(vec2) => {
                                let mut local_vec: Vec<f64> = Vec::new();
                                let (mut index_lon, mut index_lat) = (0usize, 1usize);
                                while let (Some(longitude), Some(latitude)) = (vec2.get(index_lon), vec2.get(index_lat)) {
                                    match (longitude, latitude) {
                                        (NB(x), NB(y)) => {
                                            if let (Some(x), Some(y)) = (x.as_f64(), y.as_f64()) {
                                                local_vec.push(x);
                                                local_vec.push(y);
                                            }
                                        }
                                        error => {
                                            log(format!("\nERROR : {} expected two Number bu found : {:?}", file_path.display(),error));
                                        }
                                    }
                                    (index_lon, index_lat) = (index_lon+2, index_lat+2);
                                }
                                result.push(local_vec)
                            },
                            error => {
                                log(format!("\nERROR : {} expected an Array of Number but found : {:?}", file_path.display(),error));
                                return None
                            }
                        }
                    }
                }
                error => {
                    log(format!("\nERROR : {} expected an Array of Array of Number but found : {:?}", file_path.display(),error));
                    return None
                }
            }
        }
    }
    Some(result)
}

fn split_file(clusters: &Vec<Vec<f64>>, file_path: PathBuf, result: &mut HashMap<usize, Vec<f64>>) {
    if let Ok(content) = fs::read_to_string(&file_path) {
        if let Ok(json_object) = from_str(&content) {

            process_data(clusters, json_object,result);
        }
        else { log(format!("\nERROR : can't convert into json value the content of the following file : {}",file_path.display())); }
    }
    else { log(format!("\nERROR : can't read the file : {}",file_path.display())); }
}

fn process_data(clusters: &Vec<Vec<f64>>, json_object: Value, hashmap: &mut HashMap<usize, Vec<f64>>) {
    match json_object {
        Value::Array(vec) => {
            for index in (0..vec.len()).step_by(7) {
                if let (NB(n0),NB(n1),NB(n2),NB(n3),NB(n4),NB(n5),NB(n6)) = 
                    (&vec[index], &vec[index+1], &vec[index+2], &vec[index+3], &vec[index+4], &vec[index+5], &vec[index+6])
                {
                    if let (Some(lon),Some(lat),Some(temp),Some(press),Some(winds),Some(windd),Some(waves)) = 
                    (n0.as_f64(), n1.as_f64(), n2.as_f64(), n3.as_f64(), n4.as_f64(), n5.as_f64(), n6.as_f64())
                    {
                        'outer: for index_clusters in 0..clusters.len() {
                            let local_c = &clusters[index_clusters];
                            for index_cluster in (0..(local_c).len()).step_by(2) {
                                if local_c[index_cluster] == custom_round(lon) && local_c[index_cluster+1] == custom_round(lat) {
                                    let measures = vec![lon, lat, temp, press, winds, windd, waves];
                                    if let Some(value) = hashmap.get_mut(&index_clusters) {
                                        value.extend(measures);
                                    }
                                    break 'outer;
                                }
                            }
                        }
                    }
                }
            }
        },
        _ => log("\nInconsistent format at the beguining.".to_string())
    }
}