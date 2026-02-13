//! Update README blocks from Cargo.toml; handler dispatches by block name to generators.

pub mod error;
pub mod generators;
pub mod handler;
pub mod parser;

pub use error::{Error, Result};
pub use handler::{BlockHandler, DefaultHandler, UpdateContext};
pub use parser::cargo::{ParsedManifest, parse as parse_manifest};
pub use parser::readme::{
    BlockRequest, assign_and_generate, parse_readme_blocks, replace_blocks_once, update_readme,
};

use log::info;
use std::path::Path;

pub fn run(manifest_dir: &Path, readme_path: &Path) -> Result<String> {
    run_with_handler(manifest_dir, readme_path, &DefaultHandler::default())
}

/// Custom handler: parse Cargo.toml → parse README → generate per block → one-shot replace.
pub fn run_with_handler(
    manifest_dir: &Path,
    readme_path: &Path,
    handler: &dyn BlockHandler,
) -> Result<String> {
    let config = parser::cargo::parse(manifest_dir)?;
    info!("config: {:?}", config);

    let readme_content = std::fs::read_to_string(readme_path)?;

    let context = UpdateContext::new(config);
    let requests = parser::readme::parse_readme_blocks(&readme_content);
    let generated = parser::readme::assign_and_generate(&requests, handler, &context)?;
    info!("generated: {:?}", generated);
    let updated = parser::readme::replace_blocks_once(&readme_content, &generated);
    std::fs::write(readme_path, &updated)?;
    Ok(updated)
}
