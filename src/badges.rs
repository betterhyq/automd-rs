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
