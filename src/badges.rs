use std::collections::HashSet;

/// Badges config: each field is true if that badge is enabled in README.
#[derive(Debug, Default, Clone)]
pub struct Badges {
    pub version: bool,
    pub downloads: bool,
    pub docs: bool,
    pub commit_activity: bool,
    pub repo_stars: bool,
}

/// Parse `<!-- automdrs:badges version downloads docs ... -->` from README content.
/// Returns a Badges with each listed name set to true, others false.
pub fn parse_badges_from_readme(content: &str) -> Badges {
    let mut enabled: HashSet<&str> = HashSet::new();
    for line in content.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("<!-- automdrs:badges") {
            if let Some(rest) = rest.strip_suffix("-->") {
                for name in rest.split_whitespace() {
                    enabled.insert(name);
                }
                break;
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
