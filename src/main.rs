mod badges;

use find_cargo_toml::find_from_current_dir;
use std::fs;
use c12_parser::{parse_toml, FormatOptions, Formatted};
use serde::Deserialize;
use url::Url;

/// Minimal Cargo.toml shape: only need [package] name.
#[derive(Debug, Deserialize)]
struct CargoToml {
    package: Package,
}

#[derive(Debug, Deserialize)]
struct Package {
    name: String,
    repository: String,
}

fn parse_cargo_toml() -> (String, String) {
    let cargo_toml = find_from_current_dir(".", None).next().unwrap();
    let cargo_toml_content = fs::read_to_string(cargo_toml).unwrap();
    let toml: Formatted<CargoToml> =
        parse_toml(&cargo_toml_content, Some(FormatOptions::default())).unwrap();
    let name = &toml.value.package.name;
    let repository = &toml.value.package.repository;
    return (name.to_string(), repository.to_string());
}

fn parse_github_repository_url(repository: &str) -> (String, String) {
    let url = Url::parse(repository).unwrap();
    let path = url.path();
    let parts = path.split('/').collect::<Vec<&str>>();
    let username = parts[1];
    let repository = parts[2];
    return (username.to_string(), repository.to_string());
}

fn main() {
    let (name, repository) = parse_cargo_toml();
    let version_badge = badges::generate_crate_version_badge(&name);
    let downloads_badge = badges::generate_crate_downloads_badge(&name);
    let docs_badge = badges::generate_crate_docs_badge(&name);

    let (username, repository_name) = parse_github_repository_url(&repository);

    let commit_activity_badge = badges::generate_commit_activity_badge(&username, &repository_name);
    let repo_stars_badge = badges::generate_repo_stars_badge(&username, &repository_name);
    println!("{}", version_badge);
    println!("{}", downloads_badge);
    println!("{}", docs_badge);
    println!("{}", commit_activity_badge);
    println!("{}", repo_stars_badge);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cargo_toml() {
        let (name, repository) = parse_cargo_toml();
        assert_eq!(name, "automd-rs");
        assert_eq!(repository, "https://github.com/betterhyq/automd-rs.git");
    }
}
