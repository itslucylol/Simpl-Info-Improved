use std::fs;

pub fn ensure() {
    let path = "../dist";
    fs::create_dir_all(path).unwrap();
    println!("[SIM Patcher] Created Directory: {}", path);
}