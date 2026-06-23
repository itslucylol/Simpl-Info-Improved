use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const LOGO_PATH: &str = "../vendor/fastfetch/src/logo/ascii";

fn visit_dirs(dir: &Path, acc: &mut Vec<PathBuf>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                // Pass the accumulator reference down to the next directory level
                visit_dirs(&path, acc)?;
            } else if path.is_file() {
                if path.extension().and_then(|s| s.to_str()) == Some("txt") {
                    // Store the found path instead of just printing it
                    acc.push(path);
                }
            }
        }
    }
    Ok(())
}

pub fn logo_crawl() -> io::Result<Vec<PathBuf>> {
    let mut txt_files = Vec::new();
    
    // Start the recursion, passing our mutable vector to collect results
    visit_dirs(Path::new(LOGO_PATH), &mut txt_files)?;
    
    Ok(txt_files)
}