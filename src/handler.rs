//! Generic block handler extension point for automdrs blocks.
//!
//! Business modules (badges, contributors, ...) implement `BlockHandler` and are
//! registered so the generic readme updater can replace `<!-- automdrs:NAME ... -->`
//! ... `<!-- /automdrs -->` blocks with generated content.

use crate::error::Result;
use crate::manifest::ProjectConfig;

/// Context passed to every block handler (project config, future: readme path, etc.).
#[derive(Debug, Clone)]
pub struct UpdateContext {
    pub config: ProjectConfig,
}

impl UpdateContext {
    pub fn new(config: ProjectConfig) -> Self {
        Self { config }
    }
}

/// A handler for one kind of automdrs block (e.g. `automdrs:badges`, `automdrs:contributors`).
///
/// The opening tag line is passed as-is so the handler can parse its own options
/// (e.g. `<!-- automdrs:badges version downloads -->`).
pub trait BlockHandler: Send + Sync {
    /// Tag name after `automdrs:` (e.g. `"badges"`, `"contributors"`).
    fn name(&self) -> &str;

    /// Generate replacement lines for the block body. The opening tag line and
    /// closing tag are kept; only the content between them is replaced.
    fn generate(&self, open_tag_line: &str, context: &UpdateContext) -> Result<Vec<String>>;
}
