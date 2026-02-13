//! Generic block handler: dispatches by block name to generators.
//!
//! One handler uses the `generators` module and selects the generator from the block's name.

use std::collections::HashSet;

use crate::error::Result;
use crate::generators::badges::{self as badges_gen, BadgesConfig};
use crate::generators::contributors::{self as contributors_gen, ContributorsConfig};
use crate::parser::cargo::ParsedManifest;

/// Context passed to every block handler (parsed Cargo.toml result, etc.).
#[derive(Debug, Clone)]
pub struct UpdateContext {
    pub config: ParsedManifest,
}

impl UpdateContext {
    pub fn new(config: ParsedManifest) -> Self {
        Self { config }
    }
}

/// Handler trait: generate receives block name and open tag line, returns lines to fill.
pub trait BlockHandler: Send + Sync {
    /// Generate replacement lines for the block body. Dispatcher selects generator by `block_name`.
    fn generate(
        &self,
        block_name: &str,
        open_tag_line: &str,
        context: &UpdateContext,
    ) -> Result<Vec<String>>;
}

fn parse_badges_config(open_tag: &str) -> BadgesConfig {
    let mut enabled: HashSet<&str> = HashSet::new();
    let trimmed = open_tag.trim();
    if let Some(rest) = trimmed.strip_prefix("<!-- automdrs:badges") {
        if let Some(rest) = rest.strip_suffix("-->") {
            for name in rest.split_whitespace() {
                enabled.insert(name);
            }
        }
    }
    BadgesConfig {
        version: enabled.contains("version"),
        downloads: enabled.contains("downloads"),
        docs: enabled.contains("docs"),
        commit_activity: enabled.contains("commit_activity"),
        repo_stars: enabled.contains("repo_stars"),
    }
}

/// Default handler: selects generator by block name (badges, contributors, ...).
#[derive(Debug, Default)]
pub struct DefaultHandler;

impl BlockHandler for DefaultHandler {
    fn generate(
        &self,
        block_name: &str,
        open_tag_line: &str,
        context: &UpdateContext,
    ) -> Result<Vec<String>> {
        match block_name {
            "badges" => {
                let config = parse_badges_config(open_tag_line);
                Ok(badges_gen::generate(&config, &context.config))
            }
            "contributors" => {
                let config = ContributorsConfig::default();
                Ok(contributors_gen::generate(&config, &context.config))
            }
            _ => Ok(vec![]),
        }
    }
}
