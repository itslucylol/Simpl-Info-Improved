use std::process::Command;
use std::fs::{OpenOptions, read_to_string};
use std::io::{Write, Result};

fn patch(src_path: &str, dest_path: &str) -> Result<()> {
    // 1. Read the entire source file
    let content = read_to_string(src_path)?;

    // 2. Skip the first 3 lines and collect the rest
    let remaining_content: Vec<&str> = content
        .lines()
        .skip(3)
        .collect();

    // Rejoin the lines back into a single string
    let mut cleaned_text = remaining_content.join("\n");
    
    // Ensure it ends with a newline so it appends cleanly
    if !cleaned_text.is_empty() {
        cleaned_text.push('\n');
    }

    // 3. Open the destination file in Append mode
    let mut dest_file = OpenOptions::new()
        .append(true)
        .create(true) // Creates the file if it doesn't exist
        .open(dest_path)?;

    // 4. Write the content to the bottom of the target file
    dest_file.write_all(cleaned_text.as_bytes())?;

    Ok(())
}


pub fn main() {
    //---   Remove Old Display   ---//
    Command::new("sed")
        .arg("-i")
        .arg("/# The display/q")
        .arg("../build/sim")
        .output()
        .unwrap();

    patch("../patches/distro_id.sh", "../build/sim").unwrap();
    patch("../patches/render.sh",    "../build/sim").unwrap();
}