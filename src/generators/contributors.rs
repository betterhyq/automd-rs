//! Contributors generator: given config and manifest, returns lines to fill the block.

use crate::parser::cargo::ParsedManifest;

/// Options for contributors block (placeholder for future options).
#[derive(Debug, Default, Clone)]
pub struct ContributorsConfig {}

/// Returns the lines to fill the contributors block. No parsing — caller provides config and manifest.
pub fn generate(_config: &ContributorsConfig, _manifest: &ParsedManifest) -> Vec<String> {
    vec!["<!-- contributors list will be generated here -->".to_string()]
}
