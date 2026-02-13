//! Block handler for `<!-- automdrs:contributors ... -->`.
//!
//! Placeholder: implement generation (e.g. from GitHub API or CONTRIBUTORS file)
//! and parse options from the open tag line.

use crate::error::Result;
use crate::handler::{BlockHandler, UpdateContext};

/// Block handler for `<!-- automdrs:contributors ... -->`.
#[derive(Debug, Default)]
pub struct ContributorsHandler;

impl BlockHandler for ContributorsHandler {
    fn name(&self) -> &str {
        "contributors"
    }

    fn generate(&self, _open_tag_line: &str, _context: &UpdateContext) -> Result<Vec<String>> {
        // TODO: parse options from open_tag_line, fetch/generate contributor list
        Ok(vec!["<!-- contributors list will be generated here -->".to_string()])
    }
}
