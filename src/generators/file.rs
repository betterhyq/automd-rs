//! File block generator: reads a file and fills block body (optionally wrapped in code fence).

use crate::error::{Error, Result};
use log::trace;
use std::path::Path;

/// Infers code fence language from file extension.
fn infer_lang(path: &Path) -> &'static str {
    match path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
    {
        "rs" => "rust",
        "toml" => "toml",
        "md" => "markdown",
        "json" => "json",
        "yaml" | "yml" => "yaml",
        "sh" | "bash" => "bash",
        "py" => "python",
        "js" => "javascript",
        "ts" => "typescript",
        _ => "",
    }
}

/// Generates block content by reading the file at `manifest_dir/relative_src`.
/// Wraps content in markdown code fence using `lang` if provided, else inferred from path.
pub fn generate(manifest_dir: &Path, src: &str) -> Result<Vec<String>> {
    if src.is_empty() {
        return Err(Error::BlockHandler(
            "file".to_string(),
            "missing required option: src".to_string(),
        ));
    }

    let path = manifest_dir.join(src);
    let canonical = path
        .canonicalize()
        .map_err(|e| Error::BlockHandler("file".to_string(), format!("{}: {}", src, e)))?;

    // Security: ensure path is under manifest_dir
    let manifest_canonical = manifest_dir
        .canonicalize()
        .map_err(|e| Error::BlockHandler("file".to_string(), e.to_string()))?;
    if !canonical.starts_with(&manifest_canonical) {
        return Err(Error::BlockHandler(
            "file".to_string(),
            format!("path outside crate root: {}", src),
        ));
    }

    let content = std::fs::read_to_string(&path)
        .map_err(|e| Error::BlockHandler("file".to_string(), format!("{}: {}", src, e)))?;

    let lang = infer_lang(&path);
    trace!("file block: src={:?} lang={:?}", src, lang);

    let mut lines = Vec::new();
    if !lang.is_empty() {
        lines.push(format!("```{lang}"));
    }
    for line in content.lines() {
        lines.push(line.to_string());
    }
    if !lang.is_empty() {
        lines.push("```".to_string());
    }

    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infer_lang() {
        assert_eq!(infer_lang(Path::new("foo.rs")), "rust");
        assert_eq!(infer_lang(Path::new("a/b.toml")), "toml");
        assert_eq!(infer_lang(Path::new("x")), "");
    }

    #[test]
    fn test_generate_empty_src() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let out = generate(dir, "");
        assert!(out.is_err());
    }

    #[test]
    fn test_generate_file() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let out = generate(dir, "src/main.rs").unwrap();
        assert!(!out.is_empty());
        assert_eq!(out[0], "```rust");
        assert!(out.iter().any(|l| l.contains("automd_rs")));
        assert_eq!(out.last(), Some(&"```".to_string()));
    }
}
