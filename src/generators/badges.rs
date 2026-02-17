//! Badge block generator: config + manifest → markdown lines.

use crate::parser::cargo::ParsedManifest;

use log::trace;

/// Config for badge block: which badges to show.
#[derive(Debug, Default, Clone)]
pub struct BadgesConfig {
    pub version: bool,
    pub downloads: bool,
    pub docs: bool,
    pub commit_activity: bool,
    pub repo_stars: bool,
}

pub fn generate(config: &BadgesConfig, manifest: &ParsedManifest) -> Vec<String> {
    trace!("config: {:?}", config);
    trace!("manifest: {:?}", manifest);
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
    trace!("lines: {:?}", lines);
    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::cargo::ParsedManifest;

    fn manifest() -> ParsedManifest {
        ParsedManifest {
            name: "my-crate".to_string(),
            description: "d".to_string(),
            username: "user".to_string(),
            repository_name: "repo".to_string(),
        }
    }

    #[test]
    fn test_generate_empty() {
        let config = BadgesConfig::default();
        let out = generate(&config, &manifest());
        assert_eq!(out, Vec::<String>::new());
    }

    #[test]
    fn test_generate_version_docs_repo_stars() {
        let config = BadgesConfig {
            version: true,
            downloads: false,
            docs: true,
            commit_activity: false,
            repo_stars: true,
        };
        let out = generate(&config, &manifest());
        assert_eq!(out.len(), 3);
        assert!(out[0].contains("crates/v/my-crate"));
        assert!(out[1].contains("docsrs/my-crate"));
        assert!(out[2].contains("github/stars/user/repo"));
    }

    #[test]
    fn test_generate_downloads_commit_activity() {
        let config = BadgesConfig {
            version: false,
            downloads: true,
            docs: false,
            commit_activity: true,
            repo_stars: false,
        };
        let out = generate(&config, &manifest());
        assert_eq!(out.len(), 2);
        assert!(out[0].contains("crates/d/my-crate"));
        assert!(out[1].contains("commit-activity/m/user/repo"));
    }
}
