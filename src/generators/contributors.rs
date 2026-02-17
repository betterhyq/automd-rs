//! Contributors block generator: license, author, and contrib.rocks image.

use crate::parser::cargo::ParsedManifest;

use log::trace;

/// Config for contributors block: author and license.
#[derive(Debug, Default, Clone)]
pub struct ContributorsConfig {
    pub author: String,
    pub license: String,
}

pub fn generate(config: &ContributorsConfig, manifest: &ParsedManifest) -> Vec<String> {
    trace!("config: {:?}", config);
    trace!("manifest: {:?}", manifest);
    vec![format!(
        "Published under the [{license}](./LICENSE) license.\nMade by [@{author}](https://github.com/{username}) 💛\n<br><br>\n<a href=\"https://github.com/{username}/{repository_name}/graphs/contributors\">\n<img src=\"https://contrib.rocks/image?repo={username}/{repository_name}\" />\n</a>",
        author = config.author,
        license = config.license,
        username = manifest.username,
        repository_name = manifest.repository_name
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::cargo::ParsedManifest;

    #[test]
    fn test_generate() {
        let config = ContributorsConfig {
            author: "YONGQI".to_string(),
            license: "MIT".to_string(),
        };
        let manifest = ParsedManifest {
            name: "automd-rs".to_string(),
            description: "d".to_string(),
            username: "betterhyq".to_string(),
            repository_name: "automd-rs".to_string(),
        };
        let out = generate(&config, &manifest);
        assert_eq!(out.len(), 1);
        assert!(out[0].contains("YONGQI"));
        assert!(out[0].contains("MIT"));
        assert!(out[0].contains("betterhyq/automd-rs"));
    }
}
