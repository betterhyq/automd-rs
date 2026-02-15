//! Cargo install block generator.

use crate::parser::cargo::ParsedManifest;

use log::{info, trace};

pub fn generate(manifest: &ParsedManifest) -> Vec<String> {
    trace!("config: {:?}", manifest);
    info!("cargo-install config: {:?}", manifest);
    vec![format!(
        "\n```sh\ncargo install {name}\n```\n",
        name = manifest.name
    )]
}
