use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::collections::HashMap;
use std::path::{ PathBuf };

use crate::colors::match_macro_to_ansi;
use crate::fastfetch;
use crate::fastfetch::DistroConfig;

fn append_logo_to_bash_file(var_name: &str, raw_content: &str) {
    let output_path = format!("{}/logos.sh", crate::WORKING_DIRECTORY);

    // 1. CRITICAL: Handle backslashes FIRST so they don't corrupt the control sequences like \n
    let clean_escaped = raw_content
        .replace("\\", "\\\\") // Turn single backslashes into literal double backslashes
        .replace("\n", "\\n")
        .replace("\r", "\\r")
        .replace("\t", "\\t");

    // 2. Safely close single quotes for Bash
    let bash_safe_content = clean_escaped.replace("\'", "'\\''");

    // 3. Format strictly inside single quotes
    let logo_var_line = format!("SIM_{}='{}'\n", var_name, bash_safe_content);

    // 4. Append to file
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&output_path)
        .expect("Failed to open or create the output shell script file");

    file.write_all(logo_var_line.as_bytes())
        .expect("Failed to write logo line to file");
}

pub fn logo_to_bash(file_name: &str, content: &str, master_configs: &HashMap<String, DistroConfig>) {
    // 1. Clean the filename to match our map keys perfectly
    let lookup_key = file_name
        .to_lowercase()
        .replace(".txt", "")
        .replace("_small", "")
        .trim()
        .to_string();

    let safe_var_name = lookup_key
        .replace("-", "_")
        .replace(" ", "_")
        .to_uppercase();

    // 2. Fetch colors with fallbacks
    let fallback_colors = vec!["CYAN".to_string(), "MAGENTA".to_string(), "GREEN".to_string()];
    let distro_colors = match master_configs.get(&lookup_key) {
        Some(config) => &config.colors,
        None => &fallback_colors,
    };

    // 3. Inject the clean ANSI escape strings directly into the raw content
    let mut colorized_content = content.to_string();
    for (index, color_macro) in distro_colors.iter().enumerate() {
        let token_marker = format!("${}", index + 1);
        let ansi_escape = match_macro_to_ansi(color_macro);
        colorized_content = colorized_content.replace(&token_marker, &ansi_escape);
    }

    // 4. Pass the raw multi-line string straight to the file writer
    append_logo_to_bash_file(&safe_var_name, &colorized_content);
}

pub fn build(logos_path: Vec<PathBuf>) {

    let included_logos = fastfetch::get_included_logos();

    for logo_path in logos_path {
        let name = logo_path.file_name().unwrap().to_str().unwrap_or("Unknown");
        // println!("Processing logo file: {:?}", name);
        match fs::read_to_string(&logo_path) {
            Ok(content) => {
                logo_to_bash(name, &content, &included_logos);
            }
            Err(e) => {
                eprintln!("Failed to read {:?}: {}", logo_path, e);
            }
        }
    }

}