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

fn parse_readme_content(readme_path: &str) -> String {
    let readme_content = fs::read_to_string(readme_path).unwrap();
    return readme_content;
}

/// Insert generated badge lines into README between <!-- automdrs:badges ... --> and <!-- /automdrs -->.
fn insert_badges_into_readme(readme: &str, badge_lines: &[String]) -> String {
    let open_tag = "<!-- automdrs:badges";
    let close_tag = "<!-- /automdrs -->";
    let mut out = String::new();
    let mut in_block = false;
    let mut open_line = String::new();
    for line in readme.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with(open_tag) && trimmed.ends_with("-->") {
            in_block = true;
            open_line = line.to_string();
            out.push_str(line);
            out.push('\n');
            for s in badge_lines {
                out.push_str(s);
                out.push('\n');
            }
            continue;
        }
        if in_block {
            if trimmed == close_tag.trim() {
                in_block = false;
                out.push_str(line);
                out.push('\n');
            }
            continue;
        }
        out.push_str(line);
        out.push('\n');
    }
    if out.ends_with('\n') {
        out.pop();
    }
    out
}

fn main() {
    let (name, repository) = parse_cargo_toml();
    let (username, repository_name) = parse_github_repository_url(&repository);

    let readme_content = parse_readme_content("README.md");
    let badges_config = badges::parse_badges_from_readme(&readme_content);

    let mut badge_lines: Vec<String> = Vec::new();
    if badges_config.version {
        badge_lines.push(badges::generate_crate_version_badge(&name));
    }
    if badges_config.downloads {
        badge_lines.push(badges::generate_crate_downloads_badge(&name));
    }
    if badges_config.docs {
        badge_lines.push(badges::generate_crate_docs_badge(&name));
    }
    if badges_config.commit_activity {
        badge_lines.push(badges::generate_commit_activity_badge(&username, &repository_name));
    }
    if badges_config.repo_stars {
        badge_lines.push(badges::generate_repo_stars_badge(&username, &repository_name));
    }

    let updated_readme = insert_badges_into_readme(&readme_content, &badge_lines);
    println!("{}", updated_readme);
    fs::write("README.md", updated_readme).unwrap();
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
