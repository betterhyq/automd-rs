//! Cargo add block generator.

use crate::parser::cargo::ParsedManifest;

use log::trace;

pub fn generate(manifest: &ParsedManifest) -> Vec<String> {
    trace!("config: {:?}", manifest);
    vec![format!(
        "\n```sh\ncargo add {name}\n```\n",
        name = manifest.name
    )]
}
