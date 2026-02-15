//! Dispatches by block name to generators; parses tag options and fills block body.

use crate::error::Result;
use crate::generators::badges::{self as badges_gen, BadgesConfig};
use crate::generators::cargo_install::{self as cargo_install_gen};
use crate::generators::contributors::{self as contributors_gen, ContributorsConfig};
use crate::generators::with_automdrs::{self as with_automdrs_gen, WithAutomdrsConfig};
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

fn parse_with_automdrs_config(open_tag: &str) -> WithAutomdrsConfig {
    let opts = parse_tag_options(open_tag, "with-automdrs");
    trace!("with-automdrs config: {:?}", opts);
    WithAutomdrsConfig {
        message: opts.get("message").cloned().unwrap_or_default(),
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
            "with-automdrs" => {
                trace!("parsing with-automdrs config");
                let config = parse_with_automdrs_config(open_tag_line);
                Ok(with_automdrs_gen::generate(&config))
            }
            "cargo-install" => {
                trace!("parsing cargo-install config");
                Ok(cargo_install_gen::generate(&context.config))
            }
            _ => Ok(vec![]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::cargo::ParsedManifest;

    fn context() -> UpdateContext {
        UpdateContext::new(ParsedManifest {
            name: "test-crate".to_string(),
            username: "u".to_string(),
            repository_name: "r".to_string(),
        })
    }

    #[test]
    fn test_update_context_new() {
        let ctx = context();
        assert_eq!(ctx.config.name, "test-crate");
    }

    #[test]
    fn test_generate_badges() {
        let h = DefaultHandler::default();
        let out = h
            .generate(
                "badges",
                "<!-- automdrs:badges version docs -->",
                &context(),
            )
            .unwrap();
        assert!(!out.is_empty());
        assert!(out[0].contains("crates/v/test-crate"));
    }

    #[test]
    fn test_generate_contributors() {
        let h = DefaultHandler::default();
        let out = h
            .generate(
                "contributors",
                "<!-- automdrs:contributors author=\"A\" license=\"MIT\" -->",
                &context(),
            )
            .unwrap();
        assert_eq!(out.len(), 1);
        assert!(out[0].contains("A"));
        assert!(out[0].contains("MIT"));
    }

    #[test]
    fn test_generate_with_automdrs() {
        let h = DefaultHandler::default();
        let out = h
            .generate(
                "with-automdrs",
                "<!-- automdrs:with-automdrs -->",
                &context(),
            )
            .unwrap();
        assert_eq!(out.len(), 1);
        assert!(out[0].contains("automd-rs"));
    }

    #[test]
    fn test_generate_unknown_block() {
        let h = DefaultHandler::default();
        let out = h
            .generate("unknown", "<!-- automdrs:unknown -->", &context())
            .unwrap();
        assert_eq!(out, Vec::<String>::new());
    }
}
