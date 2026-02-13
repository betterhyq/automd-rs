//! Contributors block generator (placeholder).

use crate::parser::cargo::ParsedManifest;

use log::info;

#[derive(Debug, Default, Clone)]
pub struct ContributorsConfig {
    pub author: String,
    pub license: String,
}

// Published under the [MIT](./LICENSE) license.
// Made by [@YONGQI](https://github.com/betterhyq) 💛
// <br><br>
// <a href="https://github.com/betterhyq/automd-rs/graphs/contributors">
// <img src="https://contrib.rocks/image?repo=betterhyq/automd-rs" />
// </a>

pub fn generate(config: &ContributorsConfig, manifest: &ParsedManifest) -> Vec<String> {
    info!("config: {:?}", config);
    info!("manifest: {:?}", manifest);
    vec![format!(
        "Published under the [{license}](./LICENSE) license.\nMade by [@{author}](https://github.com/{username}) 💛\n<br><br>\n<a href=\"https://github.com/{username}/{repository_name}/graphs/contributors\">\n<img src=\"https://contrib.rocks/image?repo={username}/{repository_name}\" />\n</a>",
        author = config.author,
        license = config.license,
        username = manifest.username,
        repository_name = manifest.repository_name
    )]
}
