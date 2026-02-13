//! Badge generator: given badge config and manifest, returns lines to fill the block.

use crate::parser::cargo::ParsedManifest;

/// Which badges to generate (each field true = include that badge).
#[derive(Debug, Default, Clone)]
pub struct BadgesConfig {
    pub version: bool,
    pub downloads: bool,
    pub docs: bool,
    pub commit_activity: bool,
    pub repo_stars: bool,
}

/// Returns the lines to fill the badges block. No parsing — caller provides config and manifest.
pub fn generate(config: &BadgesConfig, manifest: &ParsedManifest) -> Vec<String> {
    let mut lines = Vec::new();
    if config.version {
        lines.push(format!("![Crates.io Version](https://img.shields.io/crates/v/{})", manifest.name));
    }
    if config.downloads {
        lines.push(format!(
            "![Crates.io Total Downloads](https://img.shields.io/crates/d/{})",
            manifest.name
        ));
    }
    if config.docs {
        lines.push(format!("![docs.rs](https://img.shields.io/docsrs/{})", manifest.name));
    }
    if config.commit_activity {
        lines.push(format!(
            "![GitHub commit activity](https://img.shields.io/github/commit-activity/m/{}/{})",
            manifest.username, manifest.repository_name
        ));
    }
    if config.repo_stars {
        lines.push(format!(
            "![GitHub Repo stars](https://img.shields.io/github/stars/{}/{})",
            manifest.username, manifest.repository_name
        ));
    }
    lines
}
