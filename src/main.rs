mod dataset;
mod logs;
mod coordinates;
mod clustering;

use std::{env, process::Command};
use logs::*;

fn main() {
    clustering_coordinates();
}

#[allow(unused)]
fn clustering_coordinates() {
    use clustering;
    use std::thread;

    if let Ok(current_directory) = env::current_dir() {
        let file_path = format!("{}/DATA/Dataset/coordinates.json", current_directory.display());
        for iterations in 0..100 {
            let clusters_path = format!("{}/DATA/Clusters/clusters{}.json", current_directory.display(),iterations);
            let centroids_path = format!("{}/DATA/Clusters/centroids{}.json", current_directory.display(),iterations);
            clustering::clustering(&file_path, &clusters_path, &centroids_path, iterations);
            
            let img_path = format!("{}/DATA/Clusters/img/clusters{}.png", current_directory.display(),iterations);
            thread::spawn(move || {
                Command::new("python3").args(["./src/map.py", &clusters_path, &centroids_path, &img_path]).spawn().unwrap().wait().unwrap();
            });
        }
    }
    else {
        log("Issue with the current directory.".to_string())
    }
}

/// Function already used to clean the file that store coordinates
#[allow(unused)]
fn clean_coordinates() {
    use coordinates;

    if let Ok(current_directory) = env::current_dir() {
        let file_path = format!("{}/DATA/Dataset/coordinates.json", current_directory.display());
        coordinates::filter_outlier(&file_path)
    }
    else {
        log("Issue with the current directory.".to_string())
    }
}

/// Function already used to extract the coordinates from the dataset
#[allow(unused)]
fn extract_coordinates() {
    use coordinates;

    if let Ok(current_directory) = env::current_dir() {
        let directory = format!("{}/DATA/Dataset", current_directory.display());
        coordinates::extract_coordinates(&directory)
    }
    else {
        log("Issue with the current directory.".to_string())
    }
}

/// Function already used to parse the file from _SeaDataNet2.zip_ -> the data extracted is saved in _data12.json_
#[allow(unused)]
fn parse_files() {
    use dataset;

    if let Ok(current_directory) = env::current_dir() {
        let directory = format!("{}/DATA/SeaDataNet2", current_directory.display());
        dataset::seadatanet2(&directory)
    }
    else {
        log("Issue with the current directory.".to_string())
    }
}

/// Function already used to optimize the storage of the dataset
#[allow(unused)]
fn clean_dataset() {
    use dataset;

    if let Ok(current_directory) = env::current_dir() {
        let directory = format!("{}/DATA/", current_directory.display());
        let files = vec![
            format!("{}Meteo_France/DATA_MeteoFranceLittle.json",directory),
            format!("{}NOAA/DATA_NOAALittle.json",directory),
            format!("{}SeaDataNet/DATA_SeaDataNetLittle.json",directory),
            format!("{}NDBC/DATA_NDBCLite_1Little.json",directory),
            format!("{}NDBC/DATA_NDBCLite_2Little.json",directory),
            format!("{}NDBC/DATA_NDBCLite_3Little.json",directory),
            format!("{}NDBC/DATA_NDBCLite_4Little.json",directory),
            format!("{}NDBC/DATA_NDBCLite_5Little.json",directory),
            format!("{}NDBC/DATA_NDBCLite_6Little.json",directory),
            format!("{}NDBC/DATA_NDBCLite_7Little.json",directory),
            format!("{}NDBC/DATA_NDBCLite_8Little.json",directory),
        ];

        dataset::clean_dataset(files)
    }
    else {
        log("Issue with the current directory.".to_string())
    }
}