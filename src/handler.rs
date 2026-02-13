//! Dispatches by block name to generators; parses tag options and fills block body.

use crate::error::Result;
use crate::generators::badges::{self as badges_gen, BadgesConfig};
use crate::generators::contributors::{self as contributors_gen, ContributorsConfig};
use crate::parser::cargo::ParsedManifest;
use crate::parser::tag_options::{option_bool, parse_tag_options};

use log::trace;

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
    let opts = parse_tag_options(open_tag, "badges");
    trace!("badges config: {:?}", opts);
    BadgesConfig {
        version: option_bool(&opts, &["showCrateVersion", "version"]),
        downloads: option_bool(&opts, &["showCrateDownloads", "downloads"]),
        docs: option_bool(&opts, &["showCrateDocs", "docs"]),
        commit_activity: option_bool(&opts, &["showCommitActivity", "commit_activity"]),
        repo_stars: option_bool(&opts, &["showRepoStars", "repo_stars"]),
    }
}

fn parse_contributors_config(open_tag: &str) -> ContributorsConfig {
    let opts = parse_tag_options(open_tag, "contributors");
    trace!("contributors config: {:?}", opts);
    ContributorsConfig {
        author: opts.get("author").cloned().unwrap_or_default(),
        license: opts.get("license").cloned().unwrap_or_default(),
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
                trace!("parsing badges config");
                let config = parse_badges_config(open_tag_line);
                Ok(badges_gen::generate(&config, &context.config))
            }
            "contributors" => {
                trace!("parsing contributors config");
                let config = parse_contributors_config(open_tag_line);
                Ok(contributors_gen::generate(&config, &context.config))
            }
            _ => Ok(vec![]),
        }
    }
}
