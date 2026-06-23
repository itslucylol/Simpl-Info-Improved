use std::fs;
use std::path::{ PathBuf };

pub fn build(logos_path: Vec<PathBuf>) {
    for logo_path in logos_path {
        let name = logo_path.file_name().unwrap().to_str().unwrap_or("Unknown");
        println!("Processing logo file: {:?}", name);
        match fs::read_to_string(&logo_path) {
            Ok(content) => {
                // Here you can process the content as needed
                println!("{}", content);
            }
            Err(e) => {
                eprintln!("Failed to read {:?}: {}", logo_path, e);
            }
        }
    }
}