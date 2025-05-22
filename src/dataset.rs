//! This module contains functions to improve the storage of the dataset and adding more data by parsing text files.

use std::fs::OpenOptions;
use std::io::{Write, Read};
use std::fs::{self, DirEntry};
use std::path::PathBuf;
use serde_json::{Value, to_string, from_str};

use crate::logs::*;

/// This function optimize the storage of the dataset
pub fn clean_dataset(files: Vec<String>) {
    let mut counter = 0usize;
    for path in files {
        counter +=1;
        let save_path = format!("data{}.json",counter);
        if let Ok(content) = fs::read_to_string(String::from(&path)) {
            if let Ok(json_object) = from_str(&content) {
                let mut result: Vec<f64> = Vec::new();

                process_data(json_object, &mut result);

                if let Ok(content) = to_string(&result) {
                    if let Ok(mut file) = OpenOptions::new().create(true).truncate(true).write(true).open(String::from(&save_path)) {
                        let _ = file.write_all(content.as_bytes());
                    }
                    else {
                        log(format!("\nERROR : clean_dataset() : can't read the file : {}", save_path));
                    }
                }
                else {
                    log(format!("\nERROR : clean_dataset() : can't convert result into string : {}",path));
                }
            }
            else {
                log(format!("\nERROR : clean_dataset() : can't convert into json value the content of the following file : {}",path));
            }
        }
        else {
            log(format!("\nERROR : clean_dataset() : can't read the file : {}",path));
        }
    }
}

/// This function optimize the storage of the json object <br>
/// It transform a matrix into a simple vector <br>
/// \[\[**longitude**, **latitude**, **temperature**, **pressure**, **wind speed**, **wind direction**,**waves height**], \[...], ...] <br>
/// -> \[**longitude**, **latitude**, **temperature**, **pressure**, **wind speed**, **wind direction**,**waves height**]
fn process_data(json_object: Value, result: &mut Vec<f64>) {
    match json_object {
        Value::Array(vector) => {
            for value in vector {
                match value {
                    Value::Array(measures) => {
                        let mut counter = 0usize;
                        for measure in &measures {
                            if let Value::Number(number) = measure {
                                if let Some(number) = number.as_f64() {
                                    result.push(number);
                                    counter += 1;
                                }
                                else {
                                    log(format!("\nERROR : process_data() : Impossible to get a f64 from {}",number));
                                }
                            }
                            else {
                                log(format!("\nERROR : process_data() : Impossible to "));
                            }
                        }
                        if counter != 7 {
                            log(format!("\nERROR : process_data() : There is a vector with {} values : {:?}",counter,measures));
                        }
                    },
                    _ => log(format!("\nERROR : process_data() : Inconsistent data : {:?}",value))
                }
            }
            log("\nSuccessfully proceed the data".to_string())
        },
        _ => log("\nInconsistent format at the beguining.".to_string())
    }
}

/// This function clean the dataset stored in the folder _SeaDataNet2_
pub fn seadatanet2(folder_path: &str) {
    let mut data: Vec<f64> = Vec::new();

    match fs::read_dir(folder_path) {
        Ok(entries) => {
            let entries = entries
                .filter(|e| e.is_ok())
                .map(|x| x.unwrap())
                .collect::<Vec<DirEntry>>();

            for entry in entries {
                let file_path = entry.path();
                parse_file(&mut data, file_path);
            }
        },
        Err(error) => log(format!("\nERROR : seadatanet2() : {}",error)),
    }

    match  OpenOptions::new().write(true).create(true).truncate(true).open("data12.json") {
        Ok(mut file) => {
            match to_string(&data) {
                Ok(content) => {
                    let _ = file.write_all(content.as_bytes());
                },
                Err(error) => log(format!("\nERROR : seadatanet2() : {}",error))
            }
        },
        Err(error) => log(format!("\nERROR : seadatanet2() : {}",error))
    }
}

/// This enumeration represent the state of the parser for the _SeaDataNet2_ files
enum ParseState {
    Start,
    FirstLine,
    OtherLine,
}

impl ParseState {
    fn new() -> Self {
        ParseState::Start
    }
}

/// This function parse a file from the _SeaDataNet2_ directory and add the correct measures parsed to the main vector
fn parse_file(vector: &mut Vec<f64>, file_path: PathBuf) {
    let header = Header::from(&format!("{}",&file_path.display()));

    match OpenOptions::new().read(true).open(&file_path) {
        Ok(mut file) => {
            let mut content = String::new();
            if let Ok(_) = file.read_to_string(&mut content) {
                let content = content.split("Cruise").collect::<Vec<&str>>()[1].to_string();
                let lines = content.split("\n").collect::<Vec<&str>>();
                let mut state = ParseState::new();
                let mut coordinates: [f64;2] = [0.0,0.0];

                for line in lines {
                    match state {
                        ParseState::Start => { state = ParseState::FirstLine },
                        ParseState::FirstLine => {
                            let parsed_data = filter_parsed_line(parse_line(line));
                            match header.push_to_vec(vector, parsed_data, None) {
                                Ok(coords) => {
                                    if let Err(_) = get_coordinates(coords, &mut coordinates) {
                                        log(format!("\nERROR [{:?}] can't find the coordinate of the station from the file : {}",header,&file_path.display()));
                                        return;
                                    }
                                    else { state = ParseState::OtherLine; }
                                }
                                Err(coords) => {
                                    log(format!("\nERROR [{:?}] inconsistent first line catched in the file : {}",header,&file_path.display()));
                                    if let Err(_) = get_coordinates(coords, &mut coordinates) {
                                        log(format!("\nERROR [{:?}] can't find the coordinate of the station from the file : {}",header,&file_path.display()));
                                        return;
                                    }
                                    else { state = ParseState::OtherLine; }
                                }
                            }
                        },
                        ParseState::OtherLine => {
                            let parsed_data = filter_parsed_line(parse_line(line));
                            if let Err(_) = header.push_to_vec(vector, parsed_data, Some((coordinates[0],coordinates[1]))) {
                                log(format!("\nERROR [{:?}] inconsistent line catched in the file : {}",header,&file_path.display()));
                            }
                        }
                    }
                }
            }
            else { log(format!("\nERROR [{:?}] can't read {}",header,file_path.display())); }
        },
        Err(error) => log(format!("\nERROR [{:?}] process_format({}) : {}",header,file_path.display(),error))
    }
}

/// This function assert that the coordinates was parsed and store them in the array
fn get_coordinates(parsed_coords: Option<(f64,f64)>, current_coords: &mut [f64;2]) -> Result<(),()> {
    if let Some(coordinates) = parsed_coords {
        current_coords[0] = coordinates.0;
        current_coords[1] = coordinates.1;
        Ok(())
    }
    else { Err(()) }
}

/// This function filter the parsed vector to only keep the f64 parsed
fn filter_parsed_line(vector: Vec<Result<f64,String>>) -> Vec<f64> {
    vector.into_iter()
    .filter(|x| x.is_ok())
    .map(|x| x.unwrap())
    .collect::<Vec<f64>>()
}

/// This function parse a line from the SeaDataNet2 _.txt_ files
fn parse_line(line: &str) -> Vec<Result<f64,String>> {
    let mut result: Vec<Result<f64,String>> = Vec::new();
    let content = line.chars().collect::<Vec<char>>();
    let mut index = 0usize;
    while let Some(char) = content.get(index) {
        match char {
            ' ' => index+=1,
            '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'|'-'|'+'|'.' => {
                let (parsed, new_index) = parse_float(&content, index);
                index = new_index;
                result.push(parsed);
            }
            _ => index+=1
        }
    }
    result
}

/// This function parse try to parse a text into a f64
fn parse_float(content: &Vec<char>, index: usize) -> (Result<f64,String>, usize) {
    let mut index = index;
    let mut state = true;
    let mut buffer = String::new();
    let result = |x:String, index:usize| -> (Result<f64,String>, usize) {
        if x == "1" || x == "9" { 
            (Err(x), index)
        }
        else {
            if let Ok(y) = x.parse::<f64>() { (Ok(y), index) }
            else { (Err(x), index) } 
        }
    };
    
    while let Some(char) = content.get(index) {
        index += 1;
        match char {
            ' ' | '\t' => {
                return result(buffer,index)
            },
            '.' => {
                if state {
                    buffer.push(*char);
                    state = false;
                }
                else {
                    buffer.push('$');
                }
            }
            _ => buffer.push(*char),
        }
    }
    return result(buffer,index)
}


const INDEX_D90V0_0: [usize;7] = [1,2,6,10,7,9,11];
const INDEX_D90V0_1: [usize;5] = [1,5,2,4,6];
const INDEX_V2_0: [usize;7] = [1,2,16,13,9,10,15];
const INDEX_V2_1: [usize;5] = [10,7,3,4,9];

/// This structure represent the Header of the differents files
#[derive(Debug)]
enum Header {
    Unsupported,
    V2,
    D90V0,
}

impl Header {
    fn from(file_name: &str) -> Self {
        if file_name.contains("0000_269_D90_V0") { Header::D90V0 }
        else if file_name.contains("V2") { Header::V2 }
        else { Header::Unsupported }
    }

    fn push_to_vec(&self, vector: &mut Vec<f64>, parsed_data: Vec<f64>, coordinates: Option<(f64, f64)>) -> Result<Option<(f64,f64)>,Option<(f64,f64)>> {
        let mut result: (Vec<f64>, Option<(f64,f64)>) = (Vec::new(), None);
        let process_const: &mut dyn FnMut(&[usize], &mut Vec<f64>) = &mut |indices: &[usize], vector: &mut Vec<f64>| {
            for &index in indices {
                if let Some(value) = parsed_data.get(index) {
                    vector.push(*value);
                }
            }
        };

        let get_coordinates: &mut dyn FnMut(&mut (Vec<f64>, Option<(f64,f64)>)) = &mut |result: &mut (Vec<f64>, Option<(f64,f64)>)| {
            if let (Some(longitude), Some(latitude)) = (result.0.get(0), result.0.get(1)) {
                result.1 = Some((*longitude,*latitude))
            }
        };

        match self {
            Header::D90V0 => {
                if let Some((longitude, latitude)) = coordinates {
                    result.0.push(longitude);
                    result.0.push(latitude);
                    process_const(&INDEX_D90V0_1, &mut result.0);
                }
                else {
                    process_const(&INDEX_D90V0_0, &mut result.0);
                    get_coordinates(&mut result);
                }
            },
            Header::V2 => {
                if let Some((longitude, latitude)) = coordinates {
                    result.0.push(longitude);
                    result.0.push(latitude);
                    process_const(&INDEX_V2_1, &mut result.0);
                }
                else {
                    process_const(&INDEX_V2_0, &mut result.0);
                    get_coordinates(&mut result);
                }
            },
            Header::Unsupported => {}
        }

        if result.0.len() == 7 {
            match assert_data(result.0) {
                Ok(measures) => {
                    vector.extend(measures);
                    Ok(result.1)
                }
                Err(inconsistent_measures) => {
                    log(format!("\nERROR [{:?}] inconsistent measures catched : {:?}",self,inconsistent_measures));
                    Err(result.1)
                }
            }
            
        }
        else { Err(result.1) }
    }
}

/// This function assert that the meteorological measures are coherent.
fn assert_data(vector: Vec<f64>)-> Result<Vec<f64>,Vec<f64>> {
    let (temp, pressure, wind_dir, waves) = (vector[2], vector[3], vector[5], vector[6]);
    if temp > -20.0 && temp < 40.0
    && pressure >= 900.0 && pressure <= 1200.0
    && wind_dir >= 0.0 && wind_dir <= 360.0
    && waves >= 0.0 && waves < 30.0
    {
        Ok(vector)
    }
    else { Err(vector) }
}