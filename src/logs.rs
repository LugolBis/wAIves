use std::fs::OpenOptions;
use std::io::Write;
use std::thread;

/// A simple fonction to log the errors
pub fn log(content: String) {
    thread::spawn(move || {
        if let Ok(mut file) = OpenOptions::new().append(true).create(true).open("logs.txt") {
            let _ = file.write_all(content.as_bytes());
        }
    });
}