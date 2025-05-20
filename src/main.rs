mod dataset;
mod logs;
mod coordinates;

use std::env;
use logs::*;

fn main() {
    println!("Hello world !");
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