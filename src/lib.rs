//! automd-rs: update README blocks from Cargo.toml and handlers (badges, contributors, …).
//!
//! Generic layer: `BlockHandler` + `update_readme` replace `<!-- automdrs:NAME ... -->` blocks.
//! Business layer: each module (badges, contributors, …) implements a handler and can be extended.

pub mod badges;
pub mod contributors;
pub mod error;
pub mod handler;
pub mod readme;
pub mod toml_parser;

pub use error::{Error, Result};
pub use handler::{BlockHandler, UpdateContext};
pub use readme::{
    assign_and_generate, parse_readme_blocks, replace_blocks_once, update_readme, BlockRequest,
};
pub use toml_parser::{parse, ParsedManifest};

use std::path::Path;

/// Default block handlers (badges, contributors). Extend or replace when calling `run_with_handlers`.
pub fn default_handlers() -> (badges::BadgesHandler, contributors::ContributorsHandler) {
    (badges::BadgesHandler::default(), contributors::ContributorsHandler::default())
}

/// Run the full pipeline with default handlers: resolve config, read README, replace blocks, write back.
pub fn run(manifest_dir: &Path, readme_path: &Path) -> Result<String> {
    let (badges, contributors) = default_handlers();
    let handlers: &[&dyn BlockHandler] = &[&badges, &contributors];
    run_with_handlers(manifest_dir, readme_path, handlers)
}

/// Run with custom handlers (e.g. add your own block types or replace defaults).
///
/// Pipeline: 1) parse Cargo.toml → 2) parse README (block requests) →
/// 3) assign generators by block name & aggregate output → 4) one-shot replace.
pub fn run_with_handlers(
    manifest_dir: &Path,
    readme_path: &Path,
    handlers: &[&dyn BlockHandler],
) -> Result<String> {
    let config = toml_parser::parse(manifest_dir)?;
    let readme_content = std::fs::read_to_string(readme_path)?;
    let context = UpdateContext::new(config);
    let requests = readme::parse_readme_blocks(&readme_content);
    let generated = readme::assign_and_generate(&requests, handlers, &context)?;
    let updated = readme::replace_blocks_once(&readme_content, &generated);
    std::fs::write(readme_path, &updated)?;
    Ok(updated)
}
