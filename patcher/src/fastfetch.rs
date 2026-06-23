use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

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



#[derive(Debug)]
#[derive(Clone)]
pub struct DistroConfig {
    pub _names: Vec<String>,
    pub colors: Vec<String>,
}

pub fn parse_builtin_inc(content: &str) -> HashMap<String, DistroConfig> {
    let mut map = HashMap::new();
    
    // Split the file by structural comments or logic boundaries that separate distros
    // Since each logo entry has an inline comment like "// LainOS" right above it,
    // splitting by "//" gives us one clean block per distro.
    for block in content.split("//") {
        if !block.contains(".names") {
            continue;
        }

        let mut names = Vec::new();
        let mut colors = Vec::new();

        // 1. Isolate and parse the .names array block
        if let Some(n_start) = block.find(".names") {
            let n_section = &block[n_start..];
            if let Some(end_brace) = n_section.find('}') {
                let inner_names = &n_section[..end_brace];
                for cap in inner_names.split('"').skip(1).step_by(2) {
                    names.push(cap.to_string());
                }
            }
        }

        // 2. Isolate and parse the .colors array block
        if let Some(c_start) = block.find(".colors") {
            let c_section = &block[c_start..];
            if let Some(open_brace) = c_section.find('{') {
                if let Some(end_brace) = c_section[open_brace..].find('}') {
                    let inner_colors = &c_section[open_brace + 1 .. open_brace + end_brace];
                    
                    for token in inner_colors.split(',') {
                        let clean_token = token
                            .replace("FF_COLOR_FG_", "")
                            .replace("FF_COLOR_FG_256", "")
                            .replace("\"", "")
                            .trim()
                            .to_string();
                        
                        if !clean_token.is_empty() {
                            colors.push(clean_token);
                        }
                    }
                }
            }
        }

        // 3. Save it to our map
        if !names.is_empty() {
            let primary_key = names[0].to_lowercase();
            map.insert(primary_key, DistroConfig { _names: names, colors });
        }
    }

    map
}


pub fn get_included_logos() -> HashMap<String, DistroConfig> {
    let mut master_configs = HashMap::new();

    // Loop through characters 'a' through 'z'
    for byte in b'a'..=b'z' {
        let letter = (byte as char).to_string();
        
        // Construct path: "../vendor/fastfetch/src/logo/builtin/a.inc"
        let mut path = PathBuf::from(LOGO_PATH);
        path.push(format!("{}.inc", letter));

        // Attempt to read the file if it exists
        if path.exists() {
            match fs::read_to_string(&path) {
                Ok(content) => {
                    // Parse this specific letter's file
                    let parsed_map = parse_builtin_inc(&content);
                    
                    // Clean and sanitize the keys and color values before merging
                    for (key, mut config) in parsed_map {
                        // 1. Normalize the lookup key to match file targets cleanly
                        let clean_key = key.trim().to_lowercase();

                        // 2. Clear out explicit "FF_COLOR_BG_" remnants if any slipped past the parser
                        for color in &mut config.colors {
                            *color = color
                                .replace("FF_COLOR_BG_", "")
                                .replace("FF_COLOR_FG_", "")
                                .trim()
                                .to_string();
                        }

                        // 3. Insert cleanly into master map
                        master_configs.insert(clean_key, config);
                    }
                }
                Err(e) => {
                    eprintln!("Warning: Failed to read file {:?}: {}", path, e);
                }
            }
        }
    }

    master_configs
}