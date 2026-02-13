//! Cargo manifest parsing and project config.

use crate::error::{Error, Result};
use c12_parser::{parse_toml, FormatOptions, Formatted};
use serde::Deserialize;
use std::path::Path;

/// Minimal Cargo.toml shape: only need [package] name and repository.
#[derive(Debug, Deserialize)]
struct CargoToml {
    package: Package,
}

#[derive(Debug, Deserialize)]
struct Package {
    name: String,
    repository: String,
}

/// Project config derived from Cargo.toml and repository URL.
#[derive(Debug, Clone)]
pub struct ProjectConfig {
    pub crate_name: String,
    pub repository_url: String,
    pub github_user: String,
    pub github_repo: String,
}

/// Parse Cargo.toml content (e.g. from a file) into crate name and repository URL.
pub fn parse_cargo_toml_content(content: &str) -> Result<(String, String)> {
    let toml: Formatted<CargoToml> =
        parse_toml(content, Some(FormatOptions::default())).map_err(|e| Error::CargoParse(e.to_string()))?;
    Ok((
        toml.value.package.name.clone(),
        toml.value.package.repository.clone(),
    ))
}

/// Find and parse Cargo.toml from the given directory; returns (crate_name, repository_url).
pub fn find_and_parse_cargo_toml(dir: &Path) -> Result<(String, String)> {
    let path = find_cargo_toml::find(dir, None::<std::path::PathBuf>, None)
        .next()
        .ok_or(Error::CargoTomlNotFound)?;
    let content = std::fs::read_to_string(path)?;
    parse_cargo_toml_content(&content)
}

/// Build full project config from Cargo.toml content and parsed GitHub URL.
pub fn project_config_from_cargo_content(content: &str) -> Result<ProjectConfig> {
    let (crate_name, repository_url) = parse_cargo_toml_content(content)?;
    let (github_user, github_repo) = crate::github::parse_repository_url(&repository_url)?;
    Ok(ProjectConfig {
        crate_name,
        repository_url,
        github_user,
        github_repo,
    })
}

/// Find Cargo.toml from directory, parse it and repository URL into ProjectConfig.
pub fn project_config_from_manifest_dir(manifest_dir: &Path) -> Result<ProjectConfig> {
    let (crate_name, repository_url) = find_and_parse_cargo_toml(manifest_dir)?;
    let (github_user, github_repo) = crate::github::parse_repository_url(&repository_url)?;
    Ok(ProjectConfig {
        crate_name,
        repository_url,
        github_user,
        github_repo,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cargo_toml_content() {
        let content = r#"
[package]
name = "automd-rs"
repository = "https://github.com/betterhyq/automd-rs.git"
"#;
        let (name, repo) = parse_cargo_toml_content(content).unwrap();
        assert_eq!(name, "automd-rs");
        assert_eq!(repo, "https://github.com/betterhyq/automd-rs.git");
    }
}
