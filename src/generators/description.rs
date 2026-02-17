//! Description block generator.

use crate::parser::cargo::ParsedManifest;

use log::{info, trace};

pub fn generate(manifest: &ParsedManifest) -> Vec<String> {
    trace!("config: {:?}", manifest);
    info!("cargo-install config: {:?}", manifest);
    vec![format!(
        "\n{description}\n",
        description = manifest.description
    )]
}
