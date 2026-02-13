//! automd-rs: update README blocks from Cargo.toml and handlers.
//!
//! Handler dispatches by block name to generators (badges, contributors, …). One handler, one pipeline.

pub mod error;
pub mod generators;
pub mod handler;
pub mod parser;

pub use error::{Error, Result};
pub use handler::{BlockHandler, DefaultHandler, UpdateContext};
pub use parser::readme::{
    assign_and_generate, parse_readme_blocks, replace_blocks_once, update_readme, BlockRequest,
};
pub use parser::cargo::{parse as parse_manifest, ParsedManifest};

use std::path::Path;

/// Run the full pipeline with the default handler (dispatches by block name to generators).
pub fn run(manifest_dir: &Path, readme_path: &Path) -> Result<String> {
    run_with_handler(manifest_dir, readme_path, &DefaultHandler::default())
}

/// Run with a custom handler (e.g. extend dispatch or replace with your own).
///
/// Pipeline: 1) parse Cargo.toml → 2) parse README (block requests) →
/// 3) for each block, handler generates by name → 4) one-shot replace.
pub fn run_with_handler(
    manifest_dir: &Path,
    readme_path: &Path,
    handler: &dyn BlockHandler,
) -> Result<String> {
    let config = parser::cargo::parse(manifest_dir)?;
    let readme_content = std::fs::read_to_string(readme_path)?;
    let context = UpdateContext::new(config);
    let requests = parser::readme::parse_readme_blocks(&readme_content);
    let generated = parser::readme::assign_and_generate(&requests, handler, &context)?;
    let updated = parser::readme::replace_blocks_once(&readme_content, &generated);
    std::fs::write(readme_path, &updated)?;
    Ok(updated)
}
