pub fn match_macro_to_ansi(token: &str) -> String {
    let t = token.trim();
    
    // 1. TrueColor RGB (e.g., "RGB 41;19;182")
    if t.starts_with("RGB ") {
        let rgb_values = t.trim_start_matches("RGB ").trim();
        return format!("\x1b[38;2;{}m", rgb_values);
    }
    
    // 2. 256 Color Mode (e.g., "256 225")
    if t.starts_with("256 ") {
        let color_index = t.trim_start_matches("256 ").trim();
        return format!("\x1b[38;5;{}m", color_index);
    }

    // 3. Standard ANSI Colors (Using direct \x1b byte escapes)
    match t {
        "BLUE" => "\x1b[0;34m".to_string(),
        "LIGHT_BLUE" => "\x1b[1;34m".to_string(),
        "GREEN" => "\x1b[0;32m".to_string(),
        "LIGHT_GREEN" => "\x1b[1;32m".to_string(),
        "RED" => "\x1b[0;31m".to_string(),
        "LIGHT_RED" => "\x1b[1;31m".to_string(),
        "YELLOW" => "\x1b[0;33m".to_string(),
        "LIGHT_YELLOW" => "\x1b[1;33m".to_string(),
        "CYAN" => "\x1b[0;36m".to_string(),
        "LIGHT_CYAN" => "\x1b[1;36m".to_string(),
        "MAGENTA" => "\x1b[0;35m".to_string(),
        "BLACK" => "\x1b[0;30m".to_string(),
        "LIGHT_BLACK" => "\x1b[1;30m".to_string(),
        "WHITE" => "\x1b[0;37m".to_string(),
        _ => "\x1b[0m".to_string(),
    }
}