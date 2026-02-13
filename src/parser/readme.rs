//! README block parsing and one-pass replacement for `<!-- automdrs:NAME ... -->` blocks.

use crate::error::Result;
use crate::handler::{BlockHandler, UpdateContext};

const OPEN_PREFIX: &str = "<!-- automdrs:";
const OPEN_SUFFIX: &str = "-->";
const CLOSE_TAG: &str = "<!-- /automdrs -->";

#[derive(Debug, Clone)]
pub struct BlockRequest {
    pub name: String,
    pub open_tag_line: String,
}

/// Parses block name from line like `<!-- automdrs:badges version -->` → `badges`.
pub fn parse_block_name(line: &str) -> Option<&str> {
    let t = line.trim();
    if !t.starts_with(OPEN_PREFIX) || !t.ends_with(OPEN_SUFFIX) {
        return None;
    }
    t.strip_prefix(OPEN_PREFIX)?
        .strip_suffix(OPEN_SUFFIX)?
        .trim()
        .split_whitespace()
        .next()
}

/// Collects all automdrs block requests in document order.
pub fn parse_readme_blocks(content: &str) -> Vec<BlockRequest> {
    let mut requests = Vec::new();
    let mut in_block = false;
    for line in content.lines() {
        let t = line.trim();
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
        if t == CLOSE_TAG.trim() {
            in_block = false;
        }
    }
    requests
}

/// Runs handler per request and returns generated lines in order.
pub fn assign_and_generate(
    requests: &[BlockRequest],
    handler: &dyn BlockHandler,
    context: &UpdateContext,
) -> Result<Vec<Vec<String>>> {
    let mut out = Vec::with_capacity(requests.len());
    for req in requests {
        out.push(handler.generate(&req.name, &req.open_tag_line, context)?);
    }
    Ok(out)
}

/// Replaces block bodies with `generated` in one pass. Output order matches block order.
pub fn replace_blocks_once(content: &str, generated: &[Vec<String>]) -> String {
    let cap = content.len().saturating_add(512);
    let mut out = String::with_capacity(cap);
    let mut in_block = false;
    let mut idx = 0usize;
    for line in content.lines() {
        let t = line.trim();
        if !in_block {
            if parse_block_name(line).is_some() {
                in_block = true;
                out.push_str(line);
                out.push('\n');
                if idx < generated.len() {
                    for s in &generated[idx] {
                        out.push_str(s);
                        out.push('\n');
                    }
                }
                idx += 1;
                continue;
            }
            out.push_str(line);
            out.push('\n');
            continue;
        }
        if t == CLOSE_TAG.trim() {
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

pub fn update_readme(
    content: &str,
    handler: &dyn BlockHandler,
    context: &UpdateContext,
) -> Result<String> {
    let requests = parse_readme_blocks(content);
    let generated = assign_and_generate(&requests, handler, context)?;
    Ok(replace_blocks_once(content, &generated))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_block_name() {
        assert_eq!(
            parse_block_name("  <!-- automdrs:badges version -->  "),
            Some("badges")
        );
        assert_eq!(
            parse_block_name("<!-- automdrs:contributors -->"),
            Some("contributors")
        );
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
