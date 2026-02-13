//! Unified parsing of `<!-- automdrs:NAME key="value" ... -->` tag options.
//! Supports both key=value (quoted or unquoted) and standalone words (treated as key=true).

use std::collections::HashMap;

use log::trace;
const OPEN_PREFIX: &str = "<!-- automdrs:";
const OPEN_SUFFIX: &str = "-->";

/// Parses the option string after the block name from an open tag line.
/// Returns a map of option names to values. Standalone words become key -> "true".
///
/// # Examples
/// - `<!-- automdrs:badges version downloads -->` with name "badges" → {"version": "true", "downloads": "true"}
/// - `<!-- automdrs:contributors author="YONGQI" license="MIT" -->` → {"author": "YONGQI", "license": "MIT"}
/// - Mixed: `<!-- automdrs:badges showCrateVersion="true" docs -->` → {"showCrateVersion": "true", "docs": "true"}
pub fn parse_tag_options(open_tag_line: &str, block_name: &str) -> HashMap<String, String> {
    trace!("parsing tag options: {:?}", open_tag_line);
    trace!("block name: {:?}", block_name);
    let mut out = HashMap::new();
    let t = open_tag_line.trim();
    let inner = t
        .strip_prefix(OPEN_PREFIX)
        .and_then(|s| s.strip_suffix(OPEN_SUFFIX))
        .map(str::trim);
    let Some(inner) = inner else {
        return out;
    };
    let rest = match inner.strip_prefix(block_name) {
        Some(r) => r.trim(),
        None => return out,
    };
    for w in rest.split_whitespace() {
        if let Some((key, value)) = w.split_once('=') {
            let value = value
                .strip_prefix('"')
                .and_then(|s| s.strip_suffix('"'))
                .or_else(|| value.strip_prefix('\'').and_then(|s| s.strip_suffix('\'')))
                .unwrap_or(value);
            out.insert(key.to_string(), value.to_string());
        } else if !w.is_empty() {
            out.insert(w.to_string(), "true".to_string());
        }
    }
    trace!("out: {:?}", out);
    out
}

/// Returns true for values that mean "on" (e.g. "true", "yes", "1").
pub fn option_bool(options: &HashMap<String, String>, keys: &[&str]) -> bool {
    for key in keys {
        if let Some(v) = options.get(*key) {
            let v = v.to_lowercase();
            if v == "true" || v == "yes" || v == "1" {
                return true;
            }
            if v == "false" || v == "no" || v == "0" {
                return false;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tag_options_standalone() {
        let opts = parse_tag_options("  <!-- automdrs:badges version downloads -->  ", "badges");
        assert_eq!(opts.get("version").map(String::as_str), Some("true"));
        assert_eq!(opts.get("downloads").map(String::as_str), Some("true"));
    }

    #[test]
    fn test_parse_tag_options_key_value() {
        let opts = parse_tag_options(
            "<!-- automdrs:contributors author=\"YONGQI\" license=\"MIT\" -->",
            "contributors",
        );
        assert_eq!(opts.get("author").map(String::as_str), Some("YONGQI"));
        assert_eq!(opts.get("license").map(String::as_str), Some("MIT"));
    }

    #[test]
    fn test_parse_tag_options_mixed() {
        let opts = parse_tag_options(
            "<!-- automdrs:badges showCrateVersion=\"true\" docs -->",
            "badges",
        );
        assert_eq!(
            opts.get("showCrateVersion").map(String::as_str),
            Some("true")
        );
        assert_eq!(opts.get("docs").map(String::as_str), Some("true"));
    }

    #[test]
    fn test_option_bool() {
        let opts: HashMap<String, String> = [
            ("on".to_string(), "true".to_string()),
            ("off".to_string(), "false".to_string()),
        ]
        .into();
        assert!(option_bool(&opts, &["on"]));
        assert!(!option_bool(&opts, &["off"]));
        assert!(option_bool(&opts, &["missing", "on"]));
        assert!(!option_bool(&opts, &["missing"]));
    }

    #[test]
    fn test_option_bool_yes_no_1_0() {
        let opts: HashMap<String, String> = [
            ("y".to_string(), "yes".to_string()),
            ("n".to_string(), "no".to_string()),
            ("one".to_string(), "1".to_string()),
            ("zero".to_string(), "0".to_string()),
        ]
        .into();
        assert!(option_bool(&opts, &["y"]));
        assert!(!option_bool(&opts, &["n"]));
        assert!(option_bool(&opts, &["one"]));
        assert!(!option_bool(&opts, &["zero"]));
    }

    #[test]
    fn test_parse_tag_options_wrong_block_name() {
        let opts = parse_tag_options("<!-- automdrs:other version -->", "badges");
        assert!(opts.is_empty());
    }
}
