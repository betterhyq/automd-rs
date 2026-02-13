//! README parsing and badge block insertion.

use crate::badges;
use crate::manifest::ProjectConfig;

const OPEN_TAG: &str = "<!-- automdrs:badges";
const CLOSE_TAG: &str = "<!-- /automdrs -->";

/// Insert generated badge lines into README between `<!-- automdrs:badges ... -->` and `<!-- /automdrs -->`.
pub fn insert_badges_into_readme(readme: &str, badge_lines: &[String]) -> String {
    let mut out = String::new();
    let mut in_block = false;
    for line in readme.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with(OPEN_TAG) && trimmed.ends_with("-->") {
            in_block = true;
            out.push_str(line);
            out.push('\n');
            for s in badge_lines {
                out.push_str(s);
                out.push('\n');
            }
            continue;
        }
        if in_block {
            if trimmed == CLOSE_TAG.trim() {
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

/// Parse README content, generate badges from config, and return updated README.
pub fn update_readme_with_badges(readme_content: &str, config: &ProjectConfig) -> String {
    let badges_config = badges::parse_badges_from_readme(readme_content);
    let badge_lines = badges::generate_all(config, &badges_config);
    insert_badges_into_readme(readme_content, &badge_lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_badges_preserves_block_markers() {
        let readme = "Title\n\n<!-- automdrs:badges version -->\n<!-- /automdrs -->\n\nRest";
        let lines = vec!["![ver](https://img.shields.io/crates/v/foo)".to_string()];
        let out = insert_badges_into_readme(readme, &lines);
        assert!(out.contains("<!-- automdrs:badges version -->"));
        assert!(out.contains("![ver](https://img.shields.io/crates/v/foo)"));
        assert!(out.contains("<!-- /automdrs -->"));
        assert!(out.contains("Rest"));
    }
}
