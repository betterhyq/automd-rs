use std::collections::HashSet;

use crate::error::Result;
use crate::handler::{BlockHandler, UpdateContext};
use crate::toml_parser::ParsedManifest;

/// Badges config: each field is true if that badge is enabled in README.
#[derive(Debug, Default, Clone)]
pub struct Badges {
    pub version: bool,
    pub downloads: bool,
    pub docs: bool,
    pub commit_activity: bool,
    pub repo_stars: bool,
}

/// Parse `<!-- automdrs:badges version downloads ... -->` from the opening tag line.
fn parse_badges_from_open_tag(open_tag: &str) -> Badges {
    let mut enabled: HashSet<&str> = HashSet::new();
    let trimmed = open_tag.trim();
    if let Some(rest) = trimmed.strip_prefix("<!-- automdrs:badges") {
        if let Some(rest) = rest.strip_suffix("-->") {
            for name in rest.split_whitespace() {
                enabled.insert(name);
            }
        }
    }
    Badges {
        version: enabled.contains("version"),
        downloads: enabled.contains("downloads"),
        docs: enabled.contains("docs"),
        commit_activity: enabled.contains("commit_activity"),
        repo_stars: enabled.contains("repo_stars"),
    }
}

pub fn generate_crate_version_badge(name: &str) -> String {
    return format!("![Crates.io Version](https://img.shields.io/crates/v/{name})");
}

pub fn generate_crate_downloads_badge(name: &str) -> String {
    return format!("![Crates.io Total Downloads](https://img.shields.io/crates/d/{name})");
}

pub fn generate_crate_docs_badge(name: &str) -> String {
    return format!("![docs.rs](https://img.shields.io/docsrs/{name})");
}

pub fn generate_commit_activity_badge(username: &str, repository: &str) -> String {
    return format!("![GitHub commit activity](https://img.shields.io/github/commit-activity/m/{username}/{repository})");
}

pub fn generate_repo_stars_badge(username: &str, repository: &str) -> String {
    return format!("![GitHub Repo stars](https://img.shields.io/github/stars/{username}/{repository})");
}

/// Generate all enabled badge lines from parsed manifest and badge options.
pub fn generate_all(config: &ParsedManifest, badges: &Badges) -> Vec<String> {
    let mut lines = Vec::new();
    if badges.version {
        lines.push(generate_crate_version_badge(&config.name));
    }
    if badges.downloads {
        lines.push(generate_crate_downloads_badge(&config.name));
    }
    if badges.docs {
        lines.push(generate_crate_docs_badge(&config.name));
    }
    if badges.commit_activity {
        lines.push(generate_commit_activity_badge(&config.username, &config.repository_name));
    }
    if badges.repo_stars {
        lines.push(generate_repo_stars_badge(&config.username, &config.repository_name));
    }
    lines
}

/// Block handler for `<!-- automdrs:badges ... -->`.
#[derive(Debug, Default)]
pub struct BadgesHandler;

impl BlockHandler for BadgesHandler {
    fn name(&self) -> &str {
        "badges"
    }

    fn generate(&self, open_tag_line: &str, context: &UpdateContext) -> Result<Vec<String>> {
        let opts = parse_badges_from_open_tag(open_tag_line);
        Ok(generate_all(&context.config, &opts))
    }
}
