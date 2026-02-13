//! GitHub repository URL parsing.

use crate::error::{Error, Result};
use url::Url;

/// Parse a GitHub repository URL into (username, repository_name).
/// Supports URLs like https://github.com/user/repo or https://github.com/user/repo.git
pub fn parse_repository_url(repository: &str) -> Result<(String, String)> {
    let url = Url::parse(repository).map_err(|e| Error::InvalidRepoUrl(e.to_string()))?;
    let path = url.path();
    let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    if parts.len() < 2 {
        return Err(Error::InvalidRepoUrl(format!("expected user/repo, got: {}", repository)));
    }
    let username = parts[0];
    let repo = parts[1].strip_suffix(".git").unwrap_or(parts[1]);
    Ok((username.to_string(), repo.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_github_url() {
        let (user, repo) = parse_repository_url("https://github.com/betterhyq/automd-rs.git").unwrap();
        assert_eq!(user, "betterhyq");
        assert_eq!(repo, "automd-rs");
    }

    #[test]
    fn test_parse_github_url_no_git_suffix() {
        let (user, repo) = parse_repository_url("https://github.com/foo/bar").unwrap();
        assert_eq!(user, "foo");
        assert_eq!(repo, "bar");
    }
}
