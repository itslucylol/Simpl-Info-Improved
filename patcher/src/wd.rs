use std::fs;
use crate::WORKING_DIRECTORY;

pub fn ensure() {
    fs::create_dir_all(WORKING_DIRECTORY).unwrap();
    println!("[SIM Patcher] Created Directory: {}", WORKING_DIRECTORY);
}