mod wd;
mod fastfetch;
mod logos;

fn main() {
    wd::ensure();
    let logo_paths = fastfetch::logo_crawl();
    if let Err(e) = logo_paths {
        eprintln!("Error crawling logos: {}", e);
    } else {
        logos::build(logo_paths.expect("Failed to get logo paths"));
    }
}
