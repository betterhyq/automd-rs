use find_cargo_toml::find_from_current_dir;
use std::fs;
use c12_parser::{parse_toml, FormatOptions, Formatted};
use serde::Deserialize;

/// Minimal Cargo.toml shape: only need [package] name.
#[derive(Debug, Deserialize)]
struct CargoToml {
    package: Package,
}

#[derive(Debug, Deserialize)]
struct Package {
    name: String,
}

fn parse_cargo_toml() -> String {
    let cargo_toml = find_from_current_dir(".", None).next().unwrap();
    let cargo_toml_content = fs::read_to_string(cargo_toml).unwrap();
    let toml: Formatted<CargoToml> =
        parse_toml(&cargo_toml_content, Some(FormatOptions::default())).unwrap();
    let name = &toml.value.package.name;
    return name.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cargo_toml() {
        let name = parse_cargo_toml();
        assert_eq!(name, "automd-rs");
    }
}
