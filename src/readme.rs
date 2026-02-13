//! Pipeline: parse README → collect block requests → assign generators → one-shot replace.
//!
//! 1. Parse README: discover all `<!-- automdrs:NAME ... -->` blocks (order preserved).
//! 2. Assign generator per block by name; aggregate all generated content.
//! 3. Single replacement pass: substitute each block body with aggregated content.

use crate::error::{Error, Result};
use crate::handler::{BlockHandler, UpdateContext};

const OPEN_PREFIX: &str = "<!-- automdrs:";
const OPEN_SUFFIX: &str = "-->";
const CLOSE_TAG: &str = "<!-- /automdrs -->";

/// One block request parsed from README (name + full open tag line).
#[derive(Debug, Clone)]
pub struct BlockRequest {
    pub name: String,
    pub open_tag_line: String,
}

/// Extract block name from opening tag line, e.g. `<!-- automdrs:badges version -->` -> Some("badges").
pub fn parse_block_name(line: &str) -> Option<&str> {
    let trimmed = line.trim();
    if !trimmed.starts_with(OPEN_PREFIX) || !trimmed.ends_with(OPEN_SUFFIX) {
        return None;
    }
    let rest = trimmed
        .strip_prefix(OPEN_PREFIX)?
        .strip_suffix(OPEN_SUFFIX)?
        .trim();
    rest.split_whitespace().next()
}

/// Step 1 (after Cargo.toml): parse README and collect block requests in document order.
pub fn parse_readme_blocks(content: &str) -> Vec<BlockRequest> {
    let mut requests = Vec::new();
    let mut in_block = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if !in_block {
            if let Some(name) = parse_block_name(line) {
                in_block = true;
                requests.push(BlockRequest {
                    name: name.to_string(),
                    open_tag_line: line.to_string(),
                });
            }
            continue;
        }
        if trimmed == CLOSE_TAG.trim() {
            in_block = false;
        }
    }
    requests
}

/// Step 2: assign generator per request (by name) and aggregate generated content in order.
pub fn assign_and_generate(
    requests: &[BlockRequest],
    handlers: &[&dyn BlockHandler],
    context: &UpdateContext,
) -> Result<Vec<Vec<String>>> {
    let mut aggregated = Vec::with_capacity(requests.len());
    for req in requests {
        let handler = handlers
            .iter()
            .find(|h| h.name() == req.name)
            .ok_or_else(|| Error::BlockHandler(req.name.clone(), "no handler registered".into()))?;
        let lines = handler.generate(&req.open_tag_line, context)?;
        aggregated.push(lines);
    }
    Ok(aggregated)
}

/// Step 3: one replacement pass — substitute each block body with the corresponding generated lines.
pub fn replace_blocks_once(content: &str, generated: &[Vec<String>]) -> String {
    let mut out = String::new();
    let mut in_block = false;
    let mut block_index: usize = 0;

    for line in content.lines() {
        let trimmed = line.trim();

        if !in_block {
            if parse_block_name(line).is_some() {
                in_block = true;
                out.push_str(line);
                out.push('\n');
                if block_index < generated.len() {
                    for s in &generated[block_index] {
                        out.push_str(s);
                        out.push('\n');
                    }
                }
                block_index += 1;
                continue;
            }
            out.push_str(line);
            out.push('\n');
            continue;
        }

        if trimmed == CLOSE_TAG.trim() {
            in_block = false;
            out.push_str(line);
            out.push('\n');
        }
    }

    if out.ends_with('\n') {
        out.pop();
    }
    out
}

/// Full update: parse blocks → assign & generate → replace once. (Convenience for run.)
pub fn update_readme(
    content: &str,
    handlers: &[&dyn BlockHandler],
    context: &UpdateContext,
) -> Result<String> {
    let requests = parse_readme_blocks(content);
    let generated = assign_and_generate(&requests, handlers, context)?;
    Ok(replace_blocks_once(content, &generated))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_block_name() {
        assert_eq!(parse_block_name("  <!-- automdrs:badges version -->  "), Some("badges"));
        assert_eq!(parse_block_name("<!-- automdrs:contributors -->"), Some("contributors"));
        assert_eq!(parse_block_name("<!-- automdrs:foo a b -->"), Some("foo"));
        assert_eq!(parse_block_name("<!-- other:tag -->"), None);
        assert_eq!(parse_block_name("not a tag"), None);
    }

    #[test]
    fn test_parse_readme_blocks_order() {
        let content = "A\n<!-- automdrs:badges version -->\n<!-- /automdrs -->\nB\n<!-- automdrs:contributors -->\n<!-- /automdrs -->\n";
        let reqs = parse_readme_blocks(content);
        assert_eq!(reqs.len(), 2);
        assert_eq!(reqs[0].name, "badges");
        assert_eq!(reqs[1].name, "contributors");
    }

    #[test]
    fn test_replace_blocks_once() {
        let content = "Title\n\n<!-- automdrs:badges version -->\n<!-- /automdrs -->\n\nRest";
        let generated = vec![vec!["line1".to_string(), "line2".to_string()]];
        let out = replace_blocks_once(content, &generated);
        assert!(out.contains("<!-- automdrs:badges version -->"));
        assert!(out.contains("line1"));
        assert!(out.contains("line2"));
        assert!(out.contains("<!-- /automdrs -->"));
        assert!(out.contains("Rest"));
    }
}
