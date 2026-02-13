//! CLI: run from current dir (Cargo.toml + README.md).

use log::{info, warn};
use std::path::Path;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let manifest_dir = Path::new(".");
    let readme_path = Path::new("README.md");
    info!("manifest_dir: {:?}", manifest_dir);
    info!("readme_path: {:?}", readme_path);

    if let Err(e) = automd_rs::run(manifest_dir, readme_path) {
        warn!("error: {}", e);
        std::process::exit(1);
    }
}
