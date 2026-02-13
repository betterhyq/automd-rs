//! Badge block generator: config + manifest → markdown lines.

use crate::parser::cargo::ParsedManifest;

use log::info;

#[derive(Debug, Default, Clone)]
pub struct BadgesConfig {
    pub version: bool,
    pub downloads: bool,
    pub docs: bool,
    pub commit_activity: bool,
    pub repo_stars: bool,
}

pub fn generate(config: &BadgesConfig, manifest: &ParsedManifest) -> Vec<String> {
    info!("config: {:?}", config);
    info!("manifest: {:?}", manifest);
    let n = config.version as usize
        + config.downloads as usize
        + config.docs as usize
        + config.commit_activity as usize
        + config.repo_stars as usize;
    let mut lines = Vec::with_capacity(n);
    if config.version {
        lines.push(format!(
            "![Crates.io Version](https://img.shields.io/crates/v/{})",
            manifest.name
        ));
    }
    if config.downloads {
        lines.push(format!(
            "![Crates.io Total Downloads](https://img.shields.io/crates/d/{})",
            manifest.name
        ));
    }
    if config.docs {
        lines.push(format!(
            "![docs.rs](https://img.shields.io/docsrs/{})",
            manifest.name
        ));
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
    info!("lines: {:?}", lines);
    lines
}
