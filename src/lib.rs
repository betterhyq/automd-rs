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

use log::trace;
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
    trace!("config: {:?}", config);

    let readme_content = std::fs::read_to_string(readme_path)?;
    trace!("readme_content: {:?}", readme_content);

    let context = UpdateContext::new(config);
    trace!("context: {:?}", context);

    let requests = parser::readme::parse_readme_blocks(&readme_content);
    trace!("requests: {:?}", requests);

    let generated = parser::readme::assign_and_generate(&requests, handler, &context)?;
    trace!("generated: {:?}", generated);

    let updated = parser::readme::replace_blocks_once(&readme_content, &generated);
    trace!("updated: {:?}", updated);

    std::fs::write(readme_path, &updated)?;
    Ok(updated)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_run_with_handler() {
        let dir = std::env::temp_dir().join("automd_rs_test");
        let _ = std::fs::create_dir_all(&dir);
        let cargo_toml = dir.join("Cargo.toml");
        let readme = dir.join("README.md");
        std::fs::write(
            &cargo_toml,
            r#"
[package]
name = "test-pkg"
version = "0.1.0"
repository = "https://github.com/foo/bar.git"
"#,
        )
        .unwrap();
        std::fs::write(
            &readme,
            "Title\n\n<!-- automdrs:badges version -->\n<!-- /automdrs -->\n",
        )
        .unwrap();
        let result = run_with_handler(&dir, &readme, &crate::handler::DefaultHandler::default());
        let out = result.unwrap();
        assert!(out.contains("crates/v/test-pkg"));
        assert!(out.contains("<!-- automdrs:badges version -->"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_run() {
        let dir = std::env::temp_dir().join("automd_rs_test_run");
        let _ = std::fs::create_dir_all(&dir);
        let cargo_toml = dir.join("Cargo.toml");
        let readme = dir.join("README.md");
        std::fs::write(
            &cargo_toml,
            r#"
[package]
name = "run-pkg"
version = "0.1.0"
repository = "https://github.com/a/b.git"
"#,
        )
        .unwrap();
        std::fs::write(
            &readme,
            "Hi\n<!-- automdrs:with-automdrs -->\n<!-- /automdrs -->\n",
        )
        .unwrap();
        let result = run(&dir, &readme);
        assert!(result.is_ok());
        let out = result.unwrap();
        assert!(out.contains("automd-rs"));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
