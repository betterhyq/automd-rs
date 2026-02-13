//! automd-rs: update README badge blocks from Cargo.toml and GitHub repo.

pub mod badges;
pub mod error;
pub mod github;
pub mod manifest;
pub mod readme;

pub use error::{Error, Result};
pub use manifest::ProjectConfig;

use std::path::Path;

/// Run the full pipeline: resolve config from Cargo.toml, read README, update badges, write back.
pub fn run(manifest_dir: &Path, readme_path: &Path) -> Result<String> {
    let config = manifest::project_config_from_manifest_dir(manifest_dir)?;
    let readme_content = std::fs::read_to_string(readme_path)?;
    let updated = readme::update_readme_with_badges(&readme_content, &config);
    std::fs::write(readme_path, &updated)?;
    Ok(updated)
}
