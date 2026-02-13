//! Dispatches by block name to generators; parses tag options and fills block body.

use std::collections::HashSet;

use crate::error::Result;
use crate::generators::badges::{self as badges_gen, BadgesConfig};
use crate::generators::contributors::{self as contributors_gen, ContributorsConfig};
use crate::parser::cargo::ParsedManifest;

#[derive(Debug, Clone)]
pub struct UpdateContext {
    pub config: ParsedManifest,
}

impl UpdateContext {
    pub fn new(config: ParsedManifest) -> Self {
        Self { config }
    }
}

pub trait BlockHandler: Send + Sync {
    fn generate(
        &self,
        block_name: &str,
        open_tag_line: &str,
        context: &UpdateContext,
    ) -> Result<Vec<String>>;
}

fn parse_badges_config(open_tag: &str) -> BadgesConfig {
    let mut on: HashSet<&str> = HashSet::new();
    let t = open_tag.trim();
    if let Some(rest) = t.strip_prefix("<!-- automdrs:badges").and_then(|r| r.strip_suffix("-->")) {
        for w in rest.split_whitespace() {
            on.insert(w);
        }
    }
    BadgesConfig {
        version: on.contains("version"),
        downloads: on.contains("downloads"),
        docs: on.contains("docs"),
        commit_activity: on.contains("commit_activity"),
        repo_stars: on.contains("repo_stars"),
    }
}

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
