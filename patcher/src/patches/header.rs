use std::fs;
use std::path::Path;

/// Structures the setup process by injecting the bash shebang, 
/// custom global variables, and the logos to the top of the main script.
pub fn main() {
    let wd_path = Path::new(crate::WORKING_DIRECTORY);
    let logos_path = wd_path.join("logos.sh");
    let sim_path = wd_path.join("sim");

    // 1. Verify target script exists before proceeding
    if !sim_path.exists() {
        eprintln!("[SIM Patcher] Error: Core 'sim' file not found in working directory.");
        return;
    }

    // 2. Read existing payload contents
    let original_sim_content = fs::read_to_string(&sim_path)
        .unwrap_or_else(|_| "".to_string());
        
    let logos_content = if logos_path.exists() {
        fs::read_to_string(&logos_path).unwrap_or_else(|_| "".to_string())
    } else {
        "".to_string()
    };

    // 3. Strip any old duplicate shebang lines from the middle arrays to keep things pristine
    let clean_logos = logos_content.replace("#!/bin/bash\n", "");
    let clean_sim = original_sim_content.replace("#!/bin/bash\n", "");

    // 4. Build our master header file structure string
    let mut script_builder = String::new();
    
    script_builder.push_str("#!/bin/bash\n");
    script_builder.push_str("# SimplInfo Improved\n");
    script_builder.push_str("# ==========================================\n");
    script_builder.push_str("# SimplInfo:          https://github.com/justaguy1091-bit/simplinfo\n");
    script_builder.push_str("# SimplInfo Improved: https://github.com/itslucylol/Simpl-Info-Improved\n\n");

    // Inject our massive processed logo database variables
    if !clean_logos.is_empty() {
        script_builder.push_str("# --- SIM Embedded Logos Array ---\n");
        script_builder.push_str(&clean_logos);
        script_builder.push_str("\n");
    }

    // Append the raw core engine logic at the very bottom
    script_builder.push_str("# --- SIM Core Engine Logic ---\n");
    script_builder.push_str(&clean_sim);

    // 5. Commit everything cleanly right over the target workspace script
    match fs::write(&sim_path, script_builder) {
        Ok(_) => println!("[SIM Patcher] Structural build absolute! Header, variables, and logos baked into 'sim'."),
        Err(e) => eprint!("[SIM Patcher] Error compiling core distribution layout: {}", e),
    }
}