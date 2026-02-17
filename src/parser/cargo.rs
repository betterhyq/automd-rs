//! Cargo.toml parsing: manifest path → name, username, repository_name.

use crate::error::{Error, Result};
use c12_parser::{FormatOptions, Formatted, parse_toml};
use log::trace;
use serde::Deserialize;
use std::path::Path;
use url::Url;

/// Parsed package metadata from Cargo.toml.
#[derive(Debug, Clone)]
pub struct ParsedManifest {
    pub name: String,
    pub description: String,
    pub username: String,
    pub repository_name: String,
}

#[derive(Debug, Deserialize)]
struct CargoToml {
    package: Package,
}

#[derive(Debug, Deserialize)]
struct Package {
    name: String,
    description: String,
    repository: String,
}

fn parse_repository_url(repository: &str) -> Result<(String, String)> {
    trace!("parsing repository url: {:?}", repository);
    let url = Url::parse(repository).map_err(|e| Error::InvalidRepoUrl(e.to_string()))?;
    let path = url.path();
    let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    if parts.len() < 2 {
        return Err(Error::InvalidRepoUrl(format!(
            "expected user/repo, got: {}",
            repository
        )));
    }
    let username = parts[0];
    let repo = parts[1].strip_suffix(".git").unwrap_or(parts[1]);
    trace!("username: {:?}", username);
    trace!("repo: {:?}", repo);
    Ok((username.to_string(), repo.to_string()))
}

pub fn parse(manifest_dir: &Path) -> Result<ParsedManifest> {
    trace!("parsing cargo.toml");
    let path = find_cargo_toml::find(manifest_dir, None::<std::path::PathBuf>, None)
        .next()
        .ok_or(Error::CargoTomlNotFound)?;
    let content = std::fs::read_to_string(path)?;
    let toml: Formatted<CargoToml> = parse_toml(&content, Some(FormatOptions::default()))
        .map_err(|e| Error::CargoParse(e.to_string()))?;
    let name = toml.value.package.name;
    let description = toml.value.package.description;
    trace!("name: {:?}", name);
    trace!("repository: {:?}", toml.value.package.repository);
    let (username, repository_name) = parse_repository_url(&toml.value.package.repository)?;
    Ok(ParsedManifest {
        name,
        description,
        username,
        repository_name,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_repository_url() {
        let (user, repo) =
            parse_repository_url("https://github.com/betterhyq/automd-rs.git").unwrap();
        assert_eq!(user, "betterhyq");
        assert_eq!(repo, "automd-rs");
        let (user, repo) = parse_repository_url("https://github.com/foo/bar").unwrap();
        assert_eq!(user, "foo");
        assert_eq!(repo, "bar");
    }

    #[test]
    fn test_parse_repository_url_invalid() {
        assert!(parse_repository_url("not-a-url").is_err());
        assert!(parse_repository_url("https://github.com/onlyone").is_err());
    }

    #[test]
    fn test_parse() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let result = parse(dir);
        assert!(result.is_ok());
        let m = result.unwrap();
        assert_eq!(m.name, "automd-rs");
        assert!(!m.username.is_empty());
        assert!(!m.repository_name.is_empty());
    }

    #[test]
    fn test_parse_not_found() {
        let dir = std::env::temp_dir();
        let result = parse(&dir);
        assert!(result.is_err());
    }
}
