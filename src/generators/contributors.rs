//! Contributors block generator (placeholder).

use crate::parser::cargo::ParsedManifest;

#[derive(Debug, Default, Clone)]
pub struct ContributorsConfig {}

pub fn generate(_config: &ContributorsConfig, _manifest: &ParsedManifest) -> Vec<String> {
    vec!["<!-- contributors list will be generated here -->".to_string()]
}
