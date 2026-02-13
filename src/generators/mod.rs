//! Generators: one function per block type. Receive config + manifest, return fill strings.
//! No parsing — callers (handlers) build config from the open tag and call the generator.

pub mod badges;
pub mod contributors;
