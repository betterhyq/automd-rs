//! Block generators: (config, manifest) → lines. No parsing; handler builds config.

pub mod badges;
pub mod cargo_add;
pub mod cargo_install;
pub mod contributors;
pub mod description;
pub mod file;
pub mod with_automdrs;