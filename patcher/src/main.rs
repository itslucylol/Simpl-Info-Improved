mod wd;
mod fastfetch;
mod logos;
mod colors;
mod simplinfo;

mod patches;

const WORKING_DIRECTORY: &str = "../build";

fn main() {
    wd::ensure();

    println!("[SIM Patcher] Starting logo crawl...");
    let logo_paths = fastfetch::logo_crawl();
    if let Err(e) = logo_paths {
        eprintln!("Error crawling logos: {}", e);
    } else {
        println!("[SIM Patcher] Starting logo build...");
        logos::build(logo_paths.expect("Failed to get logo paths"));
    }
    println!("[SIM Patcher] Patching into Simplinfo...");
    simplinfo::patch();
    println!("[SIM Patcher] Finished patching into Simplinfo!");
    println!("[SIM Patcher] All done! Check the build directory: {}", WORKING_DIRECTORY);
}
