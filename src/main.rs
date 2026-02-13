//! CLI: run from current dir (Cargo.toml + README.md).

use std::path::Path;

fn main() {
    let manifest_dir = Path::new(".");
    let readme_path = Path::new("README.md");
    if let Err(e) = automd_rs::run(manifest_dir, readme_path) {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
