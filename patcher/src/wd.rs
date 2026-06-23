use std::fs;
use std::path::Path;
use crate::WORKING_DIRECTORY;

pub fn ensure() {
    let path = Path::new(crate::WORKING_DIRECTORY);

    // If the directory already exists, wipe it out completely
    if path.exists() {
        fs::remove_dir_all(path).expect("Failed to delete existing working directory");
        println!("[SIM Patcher] Cleaned old directory: {}", WORKING_DIRECTORY);
    }

    // Rebuild the directory clean from scratch
    fs::create_dir_all(path).unwrap();
    println!("[SIM Patcher] Created Fresh Directory: {}", WORKING_DIRECTORY);
}

pub fn cp(source_file_path: &str, rename_to: Option<&str>) {
    let source_path = Path::new(source_file_path);
    
    // 1. Ensure the source file actually exists before doing anything
    if !source_path.exists() {
        eprintln!("[SIM Patcher] Error: Source file does not exist: {}", source_file_path);
        return;
    }

    // 2. Determine the final file name (use the override if provided, otherwise fallback to the source name)
    let final_file_name = match rename_to {
        Some(custom_name) => std::ffi::OsString::from(custom_name),
        None => match source_path.file_name() {
            Some(name) => name.to_os_string(),
            None => {
                eprintln!("[SIM Patcher] Error: Could not extract file name from path");
                return;
            }
        },
    };

    // 3. Construct the destination path inside the working directory
    let destination_path = Path::new(WORKING_DIRECTORY).join(final_file_name);

    // 4. Perform the file copy operation
    match fs::copy(source_path, &destination_path) {
        Ok(bytes_copied) => {
            println!(
                "[SIM Patcher] Successfully copied file ({:?} bytes) to: {:?}", 
                bytes_copied, destination_path
            );
        }
        Err(e) => {
            eprintln!("[SIM Patcher] Error: Failed to copy file to destination: {}", e);
        }
    }
}